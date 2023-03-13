use http::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;

pub struct Request {
    pub path: String,
    pub query: Option<String>,
    pub method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = RequestError;

    // GET /search?name=pit&sort=1 HTTP/1.1
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
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

impl Error for RequestError {}
