# OSCRIPT (POC)

A helper-crate for making it easy to create rust-scripts (or cli apps) with arguments without having to manually parse them or define them using clap.

Automatically converts your method arguments in to a clap implementation and makes them available in your main function. Can either use oscript_async_main with tokio or oscript_main without.

Also provides automatic support for generating shell-completion scripts using the "--generate-completion" argument.

As usual with clap you also have all the usual --help flags and so on.

Makes use of triple-slash comments for both the main function and its arguments and makes them available in the --help output. 

Example:
```rust
#!/usr/bin/env -S cargo +nightly -Z script
---
[package]
edition = "2021"
version = "0.1.0"

[dependencies]
oscript = { path="oscript", features = ["tokio"] }
#oscript = { version = "0.1.1", features = ["tokio"] }
---

use std::env::{current_dir, current_exe};
use oscript::*;

#[derive(Clone,Debug,ValueEnum)]
pub enum Fruit {
    Apple,
    Banana
}

/// This is a very fancy script using clap... in main!
/// Does not currently support structs or complex enums.
#[oscript_async_main]
pub async fn main(

    /// If no clap tag then by default you get #[clap(long)]
    some_flag:bool, 

    /// Can have non-required arguments
    possibly_some_int:Option<i32>,

    /// You must select a fruit - although it would be required even without explicitly setting the clap argument here
    /// as its not an Option<Fruit>
    #[clap(required=true)]fruit:Fruit,

    #[clap(help="Can use clap attribs here")]
    some_required_string:&str, 

) {

    println!("MAGIC: {:#?}",args);
    println!("can also access them directly: {some_required_string}");
    println!("current_dir: {}", current_dir().unwrap().display());
    println!("current_exe: {}", current_exe().unwrap().display());

}

```

Example outputs:

`example-script --help`:
```
Usage: example-script <COMMAND>

Commands:
  generate-completion  Generate shell completion scripts
  run                  This is a very fancy script using clap... in main!
                       Does not currently support structs or complex enums.
  help                 Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
  ```

`example-script run --help`:
```
This is a very fancy script using clap... in main!
Does not currently support structs or complex enums.

Usage: example-script run [OPTIONS] --fruit <FRUIT> --some-required-string <SOME_REQUIRED_STRING>

Options:
      --some-flag
          If no clap tag then by default you get #[clap(long)]
      --possibly-some-int <POSSIBLY_SOME_INT>
          Can have non-required arguments
      --fruit <FRUIT>
          You must select a fruit - although it would be required even without explicitly setting the clap argument here as its not an Option<Fruit> [possible values: apple, banana]
      --some-required-string <SOME_REQUIRED_STRING>
          Can use clap attribs here
  -h, --help
          Print help

```
