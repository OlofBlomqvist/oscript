#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, FnArg, ItemFn,Pat, PatType, Type};


#[proc_macro_attribute]
pub fn oscript_async_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    oscript_main_internal(attr, item, true)
}


#[proc_macro_attribute]
pub fn oscript_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    oscript_main_internal(attr, item, false)
}

fn oscript_main_internal(attr: TokenStream, item: TokenStream, uses_async_macro: bool) -> TokenStream {
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

    if is_marked_with_async && !use_tokio {
        panic!("Your main method is marked as async but you have not enabled tokio... Try using: #[oscript_async_main]");
    } else if !is_marked_with_async && use_tokio {
        proc_macro::Span::call_site()
            .warning("#[oscript_async_main] used on non-async main will implicitly make it async! Consider marking it for clarity.")
            .emit();
    }

    let fn_name = &input.sig.ident;
    if fn_name != "main" {
        panic!("Only the `main` function can be annotated with #[oscript_main]");
    }
    let fn_args = &input.sig.inputs;
    
   
    // this just exists to make newlines work correctly for comments --> clap(help=..)
    let doc_comments: Vec<String> = input
        .attrs
        .iter()
        .flat_map(|attr| {
            if attr.path().is_ident("doc") {
                attr.to_token_stream()
                    .to_string()
                    .split("\n")
                    .filter_map(|x|{
                        x.split_once('=')
                        .and_then(|(_, value)| value.split_once(']')
                        .map(|(comment, _)| comment.trim().trim_matches('"').to_string().trim().to_string()))
                    }).collect::<Vec<String>>()
            } else {
                vec![]
            }
        })
        .collect();

    let about_text = if !doc_comments.is_empty() {
        let about = doc_comments.join("\n");
        quote! { about=#about }
    } else {
        quote! { }
    };

    let struct_name = format_ident!("O{}Args", fn_name);

    let mut struct_fields = Vec::new();
    let mut fn_arg_conversions = Vec::new();

    for arg in fn_args.iter() {
        if let FnArg::Typed(PatType { pat, attrs, ty, .. }) = arg {
            if let Pat::Ident(pat_ident) = &**pat {
                let arg_name = &pat_ident.ident;
               
                let oargs = quote! {
                    #[clap(long)] // gets overridden by any clap macro in attrs
                    #(#attrs)*
                };

                match &**ty {
                    // For references, use String as the type and handle the conversion
                    Type::Reference(_) => {
                        struct_fields.push(quote! {
                            #oargs
                            pub #arg_name: String
                        });
                        fn_arg_conversions.push(quote! {
                            let #arg_name = args.#arg_name.as_str();
                        });
                    }

                    // For all other types, use the type directly
                    _ => {
                        struct_fields.push(quote! {
                            #oargs
                            pub #arg_name: #ty
                        });
                        fn_arg_conversions.push(quote! {
                            let #arg_name = args.#arg_name.clone();
                        });
                    }
                }
            }
        }
    }

    

    let clap_quote = quote! {
        //use clap::*;
        use clap::error::Error;
        use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser};
        #[derive(clap::Parser, Debug)]
        pub struct #struct_name {
            #(#struct_fields),*
        }

        #[derive(clap::Subcommand, Debug)]
        pub enum Commands {
            /// Generate shell completion scripts
            GenerateCompletion {
                #[clap(long)]
                shell: String,
            },
            #[clap(#about_text)]
            Run(#struct_name),
        }

        #[derive(clap::Parser, Debug)]
        #[command(author, version)]
        pub struct OscriptCli {
            #[clap(subcommand)]
            pub command: Commands,
        }
    };

    let completion_logic = quote! {
        use clap_complete::Shell;
        let mut app = <OscriptCli as clap::CommandFactory>::command();
        clap_complete::generate(shell, &mut app, env!("CARGO_PKG_NAME"), &mut std::io::stdout());
    };

    let fn_block = &input.block;
    let visibility = &input.vis;

    let async_attr_marker = if use_tokio {
        quote! {
            #[tokio::main]
        }
    } else {
        quote! {}
    };
    let fn_marker = if use_tokio {
        quote! {
            async fn
        }
    } else {
        quote! {
            fn
        }
    };

    let output = quote! {
        #clap_quote
        #async_attr_marker
        #visibility #fn_marker #fn_name() {
            let cli = OscriptCli::parse();
            match cli.command {
                Commands::GenerateCompletion { shell } => {
                    let shell = shell.parse::<Shell>().expect("Invalid shell type");
                    #completion_logic
                    return;
                }
                Commands::Run(args) => {
                    #(#fn_arg_conversions)*
                    #fn_block
                }
            }
        }
    };

    TokenStream::from(output)
}
