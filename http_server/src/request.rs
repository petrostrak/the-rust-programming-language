use http::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::str::{from_utf8, Utf8Error};

pub struct Request {
    pub path: String,
    pub query: Option<String>,
    pub method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = RequestError;

    // GET /search?name=pit&sort=1 HTTP/1.1
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let req: &str = from_utf8(value)?;
        let (method, req) = get_next_word(req).ok_or(RequestError::InvalidRequest)?;
        let (path, req) = get_next_word(req).ok_or(RequestError::InvalidRequest)?;
        let (protocol, _) = get_next_word(req).ok_or(RequestError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(RequestError::InvalidProtocol);
        }

        let method = method.parse::<Method>().unwrap();

        unimplemented!()
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
