# cleverbot.io
[![Slack Status](https://slack.cleverbot.io/badge.svg)](https://slack.cleverbot.io)
[![Build Status](https://travis-ci.org/CleverbotIO/rust-cleverbot.io.svg?branch=master)](https://travis-ci.org/CleverbotIO/rust-cleverbot.io)
[![Total Downloads](https://img.shields.io/crates/d/cleverbot_io.svg)](https://crates.io/crates/cleverbot_io)
[![Latest Version](https://img.shields.io/crates/v/cleverbot_io.svg)](https://crates.io/crates/cleverbot_io/1.0.0)
[![Crates.io](https://img.shields.io/crates/l/cleverbot_io.svg)](https://crates.io/crates/cleverbot_io/1.0.0)

A Rust interface for Cleverbot.io.

## Installation
This crate works with Cargo and can be found on crates.io with a `Cargo.toml` like:

```toml
[dependencies]
cleverbot_io = "*"
```

## Usage
```rust
extern crate cleverbot_io;

use cleverbot_io::{Cleverbot};

fn main() {
    let api_user = String::from("YOUR_API_USER").to_owned();
    let api_key = String::from("YOUR_API_KEY").to_owned();

    // Use an automatically generated reference nick by using None for the third parameter.
    let mut bot = Cleverbot::new(api_user, api_key, None).unwrap();
    println!("{}", bot.say("Hello.").unwrap());

    // Or, set a nick using Some for the third parameter.
    let mut carlos = Cleverbot::new(api_user, api_key, Some(String::from("Carlos"))).unwrap();
    println!("{}", carlos.say("Why am I still talking to you?").unwrap());
}
```
