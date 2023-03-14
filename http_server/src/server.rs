use http::StatusCode;

use crate::request::Request;
use crate::response::Response;
use std::convert::TryFrom;
use std::{io::Read, io::Write, net::TcpListener};

pub struct Server {
    pub addr: String,
    pub port: u32,
}

impl Server {
    pub fn new(port: u32) -> Self {
        let addr = "127.0.0.1".to_string();
        Self { addr, port }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.addr, self.port)).unwrap();
        println!("Listening on {:?}:{}", self.addr, self.port);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..]) {
                                Ok(req) => Response::new(
                                    StatusCode::OK,
                                    Some("<h1>It works!</h1>".to_string()),
                                ),
                                Err(e) => {
                                    println!("Failed to parse the request: {}", e);
                                    Response::new(StatusCode::BAD_REQUEST, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connection {}.", e);
                            Response::new(StatusCode::BAD_REQUEST, None).send(&mut stream);
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
