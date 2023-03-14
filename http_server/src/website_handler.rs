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

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &crate::request::Request) -> Response {
        match req.method() {
            &Method::GET => match req.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::OK, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::OK, Some(content)),
                    None => Response::new(StatusCode::NOT_FOUND, None),
                },
            },
            _ => Response::new(StatusCode::NOT_FOUND, None),
        }
    }
}
