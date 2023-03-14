#![allow(dead_code)]

use server::Server;
use std::env;
use utils::parse_port;
use website_handler::WebsiteHandler;

mod query_string;
mod request;
mod response;
mod server;
mod utils;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path: {}", public_path);
    let server = Server::new(parse_port());
    server.run(WebsiteHandler::new(public_path));
}
