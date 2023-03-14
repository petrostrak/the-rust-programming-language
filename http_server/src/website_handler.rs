use std::fs;

use http::{Method, StatusCode};

use crate::{response::Response, server::Handler};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_files(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &crate::request::Request) -> Response {
        match req.method() {
            &Method::GET => match req.path() {
                "/" => Response::new(StatusCode::OK, self.read_files("index.html")),
                "/hello" => Response::new(StatusCode::OK, self.read_files("hello.html")),
                _ => Response::new(StatusCode::NOT_FOUND, None),
            },
            _ => Response::new(StatusCode::NOT_FOUND, None),
        }
    }
}
