use http::Method;
use std::io::stdin;
use std::net::{IpAddr, Ipv4Addr};

struct Server {
    addr: IpAddr,
    port: u32,
}

impl Server {
    fn new(port: u32) -> Self {
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        Self { addr, port }
    }

    fn run(&self) {
        println!("Listening on {:?}.{}", self.addr, self.port)
    }
}

struct Request {
    path: String,
    query: Option<String>,
    method: Method,
}

fn main() {
    let server = Server::new(parse_port());
    server.run();
}

fn parse_port() -> u32 {
    println!("Please give the port to launch the server:");
    let mut port: String = String::new();
    stdin().read_line(&mut port).unwrap();
    let p: u32 = port.trim().parse::<u32>().unwrap();
    p
}
