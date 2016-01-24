// Copyright (c) 2016 The Rouille developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use rustc_serialize::Decodable;
use rustc_serialize::json;
use std::string::FromUtf8Error;
use Request;

/// Error that can happen when parsing the JSON input.
#[derive(Debug)]
pub enum JsonError {
    /// Wrong content type.
    WrongContentType,

    /// The request's body was not UTF8.
    NotUtf8(FromUtf8Error),

    /// Error while parsing.
    ParseError(json::DecoderError),
}

impl From<FromUtf8Error> for JsonError {
    fn from(err: FromUtf8Error) -> JsonError {
        JsonError::NotUtf8(err)
    }
}

impl From<json::DecoderError> for JsonError {
    fn from(err: json::DecoderError) -> JsonError {
        JsonError::ParseError(err)
    }
}

pub fn get_json_input<O>(request: &Request) -> Result<O, JsonError> where O: Decodable {
    // TODO: slow
    if let Some(header) = request.header("Content-Type"){
        if !header.starts_with("application/json") {
            return Err(JsonError::WrongContentType);
        }
    } else {
        return Err(JsonError::WrongContentType);
    }

    let content = try!(String::from_utf8(request.data()));
    let data = try!(json::decode(&content));
    Ok(data)
}
