# OSCRIPT (POC)

A helper-crate for making it easy to create rust-scripts with arguments without having to manually parse them or define them using clap.

Automatically converts your method arguments in to a clap implementation and makes them available in your main function. Can either use oscript_async_main with tokio or oscript_main without.

Also provides automatic support for generating shell-completion scripts using the "--generate-completion" argument.

As usual with clap you also have all the usual --help flags and so on.


Example:
```rust
#!/usr/bin/env -S cargo +nightly -Z script
---
[package]
edition = "2021"
version = "0.1.0"

[dependencies]
oscript = { version = "0.1.0", features = ["tokio"] }
---

use std::env::{current_dir, current_exe};
use oscript::*;

#[oscript_async_main]
pub fn main(some_required_string:&str, some_flag:&bool, possibly_some_int:Option<i32>) {

    println!("MAGIC: {:#?}",args);
    println!("can also access them directly: {some_required_string}");
    println!("current_dir: {}", current_dir().unwrap().display());
    println!("current_exe: {}", current_exe().unwrap().display());

}
```