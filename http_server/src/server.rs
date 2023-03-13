use std::{io::Read, net::TcpListener};

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
                        Ok(_) => println!("Received a request: {}", String::from_utf8_lossy(&buf)),
                        Err(e) => println!("Failed to read from connection {}.", e),
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
