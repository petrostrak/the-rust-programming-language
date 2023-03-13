#![allow(dead_code)]

use server::Server;
use utils::parse_port;

mod query_string;
mod request;
mod server;
mod utils;

fn main() {
    let server = Server::new(parse_port());
    server.run();
}
