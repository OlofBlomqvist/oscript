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