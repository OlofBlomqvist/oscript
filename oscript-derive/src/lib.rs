#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatType, Type};


#[proc_macro_attribute]
pub fn oscript_async_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    oscript_main_internal(attr, item, true)
}


#[proc_macro_attribute]
pub fn oscript_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    oscript_main_internal(attr, item, false)
}

fn oscript_main_internal(attr: TokenStream, item: TokenStream,uses_async_macro:bool) -> TokenStream {
    
    let mut use_tokio = uses_async_macro;

    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("use_tokio") {
            use_tokio = true;
            Ok(())
        } else {
            Err(meta.error("unsupported attribute for [oscript..] on your main method.."))
        }
    });
    
    parse_macro_input!(attr with parser);

    let input = parse_macro_input!(item as ItemFn);

    let is_marked_with_async = input.sig.asyncness.is_some();

    if is_marked_with_async && use_tokio == false {
        panic!("Your main method is marked as async but you have not enabled tokio... Try using: #[oscript_async_main]")
    }
    else if !is_marked_with_async && use_tokio  {
        proc_macro::Span::call_site().warning("#[oscript_async_main] used on non-async main will implicitly make it async! Consider marking it for clarity.").emit();
    }

    // Extract function name and arguments
    let fn_name = &input.sig.ident;
    if fn_name != "main" {
        panic!("Only the `main` function can be annotated with #[oscript_main]");
    }
    let fn_args = &input.sig.inputs;

    let struct_name = format_ident!("O{}Args", fn_name);

    let mut struct_fields = Vec::new();
    let mut fn_arg_names = Vec::new();
    let mut fn_arg_conversions = Vec::new();

    for arg in fn_args.iter() {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            if let Pat::Ident(pat_ident) = &**pat {
                let arg_name = &pat_ident.ident;
                match &**ty {
                    Type::Reference(_) => {
                        // If the argument is a reference we will just use string.. not sure what else to do atm
                        struct_fields.push(quote! {
                            #[clap(long)]
                            pub #arg_name: String
                        });
                        fn_arg_names.push(arg_name.clone());
                        fn_arg_conversions.push(quote! {
                            let #arg_name = args.#arg_name.as_str();
                        });
                    }
                    _ => {
                        struct_fields.push(quote! {
                            #[clap(long)]
                            pub #arg_name: #ty
                        });
                        fn_arg_names.push(arg_name.clone());
                        fn_arg_conversions.push(quote! {
                            let #arg_name = args.#arg_name.clone();
                        });
                    }
                }
            }
        }
    }

    // Add a custom flag for generating shell script IntelliSense
    struct_fields.push(quote! {
        #[clap(long, hide = false)]
        pub generate_completion: Option<String>
    });

    // Handle IntelliSense flag logic so we can help people generate script completion :D
    let completion_logic = quote! {
        use clap_complete::Shell;
        if let Some(shell) = &args.generate_completion {
            let mut app = <#struct_name as clap::CommandFactory>::command();
            let shell = shell.parse::<Shell>().expect("Invalid shell type");
            clap_complete::generate(shell, &mut app, env!("CARGO_PKG_NAME"), &mut std::io::stdout());
            return;
        }
    };

    // Remove arguments from the function since main cant have them
    let fn_block = &input.block;
    let visibility = &input.vis;

    let clap_quote = quote! {
        #[derive(clap::Parser, Debug)]
        pub struct #struct_name {
            #(#struct_fields),*
        }
    };

    // probably just join these if there is no more diff than this..
    let output = if use_tokio {
        quote! {
            use clap_complete::Shell;
            #clap_quote
            #[tokio::main]
            #visibility async fn #fn_name() {
                let args = <#struct_name as clap::Parser>::parse();
                #completion_logic
                #(#fn_arg_conversions)*
                #fn_block
            }
        }
    } else {
        quote! {
            #clap_quote
            #visibility fn #fn_name() {
                let args = <#struct_name as clap::Parser>::parse();
                #completion_logic
                #(#fn_arg_conversions)*
                #fn_block
            }
        }
    };

    TokenStream::from(output)
}
