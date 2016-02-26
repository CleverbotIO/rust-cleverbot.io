extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

use hyper::client::{Client};
use hyper::header::{ContentType};
use hyper::client::response::{Response};
use url::form_urlencoded::{serialize};
use std::io::Read;
use std::collections::BTreeMap;

pub struct Cleverbot {
    user: String,
    key: String,
    pub nick: String,
}

impl Cleverbot {
    /// Creates a new Cleverbot instance.
    /// * `user` - The API User.
    /// * `key` - The API Key.
    pub fn new(user: String, key: String, nick: Option<String>) -> Result<Cleverbot, String> {
        let mut response = {
            let mut args = vec![
                ("user", &*user),
                ("key", &*key),
            ];
            if let Some(ref nick) = nick { args.push(("nick", &*nick)) }
            request("https://cleverbot.io/1.0/create", &*args)
        };
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();
        let json: BTreeMap<String, String> = serde_json::from_str(&body).unwrap();
        let result = json.get("status").unwrap();
        if result == "success" {
            Ok(Cleverbot {
                user: user,
                key: key,
                nick: json.get("nick").unwrap().to_string(),
            })
        } else {
            Err(result.to_string())
        }
    }

    /// Sends the bot a message and returns its response. If the nick is not set, it will
    /// set it randomly through set_nick_randomly. Returns its response or error string.
    /// * `message` - The message to send to the bot.
    pub fn say(&mut self, message: &str) -> Result<String, String> {
        let args = vec![
            ("user", &*self.user),
            ("key", &*self.key),
            ("nick", &*self.nick),
            ("text", message),
        ];
        let mut response = request("https://cleverbot.io/1.0/ask", &*args);
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();
        let json: BTreeMap<String, String> = serde_json::from_str(&body).unwrap();
        let result = json.get("status").unwrap();
        if result == "success" {
            return Ok(json.get("response").unwrap().to_string());
        } else {
            return Err(result.to_string());
        }
    }
}

/// Submits a POST request to the URL with the given vec body.
/// * `base` - The URL
/// * `args` - A vector representing the request body.
fn request(base: &str, args: &[(&str, &str)]) -> Response {
    let client = Client::new();
    let body = serialize(args.into_iter());
    client.post(base)
        .body(&body)
        .header(ContentType::form_url_encoded())
        .send()
        .unwrap()
}
