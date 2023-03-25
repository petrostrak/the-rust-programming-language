use std::thread;

fn main() {
    let data = vec!['a', 'b', 'c'];
    let cap = thread::spawn(move || {
        data.iter()
            .map(|v| v.to_ascii_uppercase())
            .collect::<Vec<char>>()
    });

    println!("Waiting for value...");

    match cap.join() {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("error joining thread: {:?}", e),
    }
}
