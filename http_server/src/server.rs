use std::net::{IpAddr, Ipv4Addr};

pub struct Server {
    pub addr: IpAddr,
    pub port: u32,
}

impl Server {
    pub fn new(port: u32) -> Self {
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        Self { addr, port }
    }

    pub fn run(&self) {
        println!("Listening on {:?}:{}", self.addr, self.port)
    }
}
