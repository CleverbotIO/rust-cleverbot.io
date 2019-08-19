# cleverbot.io
[![Slack Status](https://slack.cleverbot.io/badge.svg)](https://slack.cleverbot.io)
[![Build Status](https://travis-ci.org/CleverbotIO/rust-cleverbot.io.svg?branch=master)](https://travis-ci.org/CleverbotIO/rust-cleverbot.io)
[![Total Downloads](https://img.shields.io/crates/d/cleverbot_io.svg)](https://crates.io/crates/cleverbot_io)
[![Latest Version](https://img.shields.io/crates/v/cleverbot_io.svg)](https://crates.io/crates/cleverbot_io/1.0.0)
[![Crates.io](https://img.shields.io/crates/l/cleverbot_io.svg)](https://crates.io/crates/cleverbot_io/1.0.0)

A Rust interface for Cleverbot.io.

**Important**: For all I know, the cleverbot.io API does not work anymore. This crate, therefore, will not be maintained. Though this project is part of the Cleverbot IO project, I have nothing to do with and am not responsible for the maintenance of the Cleverbot IO API itself. The maintainer of the API has gone MIA. If your application still requires this crate, you should remove it.

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
    let api_user = "YOUR_API_USER";
    let api_key = "YOUR_API_KEY";

    let mut bot = Cleverbot::new(api_user.into(), api_key.into(), None).unwrap();
    println!("{}", bot.say("Hello.").unwrap());

    let mut carlos = Cleverbot::new(api_user.into(), api_key.into(), Some(String::from("Carlos1"))).unwrap();
    println!("{}", carlos.say("Why am I still talking to you?").unwrap());
}
```
