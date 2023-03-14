use http::StatusCode;

use crate::{response::Response, server::Handler};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &crate::request::Request) -> Response {
        Response::new(StatusCode::OK, Some("<h1>Hello Rust!</h1>".to_string()))
    }
}
