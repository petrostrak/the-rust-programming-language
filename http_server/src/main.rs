#![allow(dead_code)]

use server::Server;
use utils::parse_port;
use website_handler::WebsiteHandler;

mod query_string;
mod request;
mod response;
mod server;
mod utils;
mod website_handler;

fn main() {
    let server = Server::new(parse_port());
    server.run(WebsiteHandler);
}
