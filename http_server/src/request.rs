use http::Method;
use std::convert::TryFrom;

pub struct Request {
    pub path: String,
    pub query: Option<String>,
    pub method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
