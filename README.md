# cleverbot.io
<!--
TRAVIS HERE
CRATES IO HERE
-->
[![build status](https://secure.travis-ci.org/CleverbotIO/rust-cleverbot.io.svg)](http://travis-ci.org/CleverbotIO/rust-cleverbot.io)
[![Slack Status](https://slack.cleverbot.io/badge.svg)](https://slack.cleverbot.io)

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
    // Use an automatically generated reference nick by using None for the third parameter.
    let mut bot = Cleverbot::new(String::from("YOUR_API_KEY"), String::from("YOUR_API_USER"), None);
    // Print the bot's response to a greeting.
    println!("{}", bot.say(&"Hello."));

    // Or, set a nick using Some for the third parameter.
    let mut bot1 = Cleverbot::new(String::from("YOUR_API_KEY"), String::from("YOUR_API_USER"), Some(String::from("Carlos")));
    println!("{}", bot1.say(&"Why am I still talking to you?"));
}

```
