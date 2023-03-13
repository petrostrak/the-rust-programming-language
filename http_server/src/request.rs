use http::Method;

pub struct Request {
    pub path: String,
    pub query: Option<String>,
    pub method: Method,
}
