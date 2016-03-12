extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

use hyper::client::{Client};
use hyper::header::{ContentType};
use hyper::client::response::{Response};
use hyper::error::Error as HyperError;
use url::form_urlencoded::{serialize};
use std::io::Read;
use std::io::Error;
use std::collections::BTreeMap;
use serde_json::error::Error as JsonError;

#[derive(Debug)]
pub enum CleverbotError {
    IncorrectCredentials,
    DuplicatedReferenceNames,
    Io(HyperError),
    Api(String),
    Std(Error),
    Json(JsonError),
    MissingValue(String)
}

impl From<HyperError> for CleverbotError {
    fn from(err: HyperError) -> CleverbotError {
        CleverbotError::Io(err)
    }
}

impl From<Error> for CleverbotError {
    fn from(err: Error) -> CleverbotError {
        CleverbotError::Std(err)
    }
}

impl From<JsonError> for CleverbotError {
    fn from(err: JsonError) -> CleverbotError {
        CleverbotError::Json(err)
    }
}

pub struct Cleverbot {
    user: String,
    key: String,
    pub nick: String,
}

impl Cleverbot {
    /// Creates a new Cleverbot instance.
    /// * `user` - The API User.
    /// * `key` - The API Key.
    pub fn new(user: String, key: String, nick: Option<String>) -> Result<Cleverbot, CleverbotError> {
        let mut response = {
            let mut args = vec![
                ("user", &*user),
                ("key", &*key),
            ];
            if let Some(ref nick) = nick { args.push(("nick", &*nick)) }
            match request("https://cleverbot.io/1.0/create", &*args) {
                Ok(response) => response,
                Err(err) => return Err(CleverbotError::Io(err)),
            }
        };
        let mut body = String::new();
        try!(response.read_to_string(&mut body));
        let json: BTreeMap<String, String> = try!(serde_json::from_str(&body));
        let opt_result = json.get("status");
        let result = {
            if let Some(opt_result) = opt_result {
                opt_result.to_owned()
            } else {
                return Err(CleverbotError::MissingValue(String::from("status")));
            }
        };
        match result.as_ref() {
            "success" => {
                let json_nick = json.get("nick");
                let nick = {
                    if let Some(ref json_nick) = json_nick {
                        json_nick
                    } else {
                        return Err(CleverbotError::MissingValue(String::from("nick")));
                    }
                };
                Ok(Cleverbot {
                    user: user,
                    key: key,
                    nick: nick.to_string(),
                })
            },
            "Error: API credentials incorrect" => Err(CleverbotError::IncorrectCredentials),
            "Error: reference name already exists" => Err(CleverbotError::DuplicatedReferenceNames),
            _ => Err(CleverbotError::Api(result))
        }
    }

    /// Sends the bot a message and returns its response. If the nick is not set, it will
    /// set it randomly through set_nick_randomly. Returns its response or error string.
    /// * `message` - The message to send to the bot.
    pub fn say(&mut self, message: &str) -> Result<String, CleverbotError> {
        let args = vec![
            ("user", &*self.user),
            ("key", &*self.key),
            ("nick", &*self.nick),
            ("text", message),
        ];
        let mut response = {
            match request("https://cleverbot.io/1.0/ask", &*args) {
                Ok(response) => response,
                Err(err) => return Err(CleverbotError::Io(err)),
            }
        };
        let mut body = String::new();
        try!(response.read_to_string(&mut body));
        let json: BTreeMap<String, String> = try!(serde_json::from_str(&body));
        let opt_result = json.get("status");
        let result = {
            if let Some(opt_result) = opt_result {
                opt_result.to_owned()
            } else {
                return Err(CleverbotError::MissingValue(String::from("status")));
            }
        };
        match result.as_ref() {
            "success" => {
                let json_response = json.get("response");
                let response = {
                    if let Some(ref json_response) = json_response {
                        json_response
                    } else {
                        return Err(CleverbotError::MissingValue(String::from("nick")));
                    }
                };
                Ok(response.to_string())
            },
            "Error: API credentials incorrect" => Err(CleverbotError::IncorrectCredentials),
            "Error: reference name already exists" => Err(CleverbotError::DuplicatedReferenceNames),
            _ => Err(CleverbotError::Api(result.to_owned()))
        }
    }
}

/// Submits a POST request to the URL with the given vec body.
/// * `base` - The URL
/// * `args` - A vector representing the request body.
fn request(base: &str, args: &[(&str, &str)]) -> Result<Response, HyperError> {
    let client = Client::new();
    let body = serialize(args.into_iter());
    client.post(base)
        .body(&body)
        .header(ContentType::form_url_encoded())
        .send()
}
