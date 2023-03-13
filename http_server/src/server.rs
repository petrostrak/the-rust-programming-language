use std::net::TcpListener;

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
        let listener = TcpListener::bind(format!("{}:{}", self.addr, self.port));
        println!("Listening on {:?}:{}", self.addr, self.port)
    }
}
