use http::{Method, StatusCode};

use crate::{response::Response, server::Handler};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &crate::request::Request) -> Response {
        match req.method() {
            &Method::GET => match req.path() {
                "/" => Response::new(StatusCode::OK, Some("<h1>Welcome!</h1>".to_string())),
                "/hello" => Response::new(StatusCode::OK, Some("<h1>Hello Rust!</h1>".to_string())),
                _ => Response::new(StatusCode::NOT_FOUND, None),
            },
            _ => Response::new(StatusCode::NOT_FOUND, None),
        }
    }
}
