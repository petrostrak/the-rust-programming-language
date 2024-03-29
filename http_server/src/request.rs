use http::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::str::{from_utf8, Utf8Error};

use crate::query_string::QueryString;

#[derive(Debug)]
pub struct Request<'buf> {
    pub path: &'buf str,
    pub query: Option<QueryString<'buf>>,
    pub method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = RequestError;

    // GET /search?name=pit&sort=1 HTTP/1.1
    fn try_from(value: &'buf [u8]) -> Result<Self, Self::Error> {
        let req: &str = from_utf8(value)?;
        let (method, req) = get_next_word(req).ok_or(RequestError::InvalidRequest)?;
        let (mut path, req) = get_next_word(req).ok_or(RequestError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(RequestError::InvalidRequest)?;

        // if protocol != "HTTP/1.1" {
        //     return Err(RequestError::InvalidProtocol);
        // }

        let method = method.parse::<Method>().unwrap();

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Request {
            path,
            query: query_string,
            method,
        })
    }
}

#[derive(Debug)]
pub enum RequestError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl RequestError {
    fn message(&self) -> &str {
        match self {
            RequestError::InvalidRequest => "Invalid request",
            RequestError::InvalidEncoding => "Invalid encoding",
            RequestError::InvalidProtocol => "Invalid protocol",
            RequestError::InvalidMethod => "Invalid method",
        }
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for RequestError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for RequestError {}

fn get_next_word(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c == ' ' || c == '\n' {
            return Some((&req[..i], &req[i + 1..]));
        }
    }
    None
}
