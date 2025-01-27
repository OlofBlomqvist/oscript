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
oscript = { version = "0.1.1", features = ["tokio"] }
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




Another example:
```rust
#!/usr/bin/env -S cargo +nightly -Z script
--- 
[package]
name = "atest"
version = "0.1.0"
[dependencies]
argon2 = { version = "0.5.3" }
oscript = { version = "0.1.1" }
---
use argon2::{password_hash::SaltString, Argon2, Params, PasswordHasher, PasswordVerifier};
use oscript::*;

#[oscript_main]
fn main(pwd:&str,salt:&str,pepper:&str) {

    let before = std::time::Instant::now();

    let b64_salt = SaltString::encode_b64(salt.as_bytes())
        .expect("salt must be less than 64 bytes long");
 
    let argon2 = Argon2::new_with_secret(
        pepper.as_bytes(),            // Secret/Pepper
        argon2::Algorithm::Argon2id,  // Recommended 
        argon2::Version::V0x13,       // Latest version
        Params::new(
            1024*19,     // - `m_cost`: memory size in 1 KiB blocks. Between 8\*`p_cost` and (2^32)-1.
            2,           // - `t_cost`: number of iterations. Between 1 and (2^32)-1.
            1,           // - `p_cost`: degree of parallelism. Between 1 and (2^24)-1.
            Some(32) // - `output_len`: size of the KDF output in bytes. Default 32.
        ).unwrap()).unwrap();

    let hash = argon2.hash_password(pwd.as_bytes(), &b64_salt).unwrap();  
    let serialized_hash = hash.to_string();

    println!("Serialized result: {serialized_hash}");
 
    match argon2.verify_password(pwd.as_bytes(), &hash) {
        Ok(_) => println!("Yep, it works"),
        Err(e) => println!("Nope, no works: {e:?}"),
    }

    println!("TIME!!!! {:.2?}", before.elapsed());

}
```
