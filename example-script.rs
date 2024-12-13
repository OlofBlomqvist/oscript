#!/usr/bin/env -S cargo +nightly -Z script
---
[package]
edition = "2021"
version = "0.1.0"

[dependencies]
oscript = { path = "oscript", features = ["tokio"] }
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
