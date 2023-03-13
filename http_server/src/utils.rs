use std::io::stdin;

pub fn parse_port() -> u32 {
    println!("Please give the port to launch the server:");
    let mut port: String = String::new();
    stdin().read_line(&mut port).unwrap();
    let p: u32 = port.trim().parse::<u32>().unwrap();
    p
}
