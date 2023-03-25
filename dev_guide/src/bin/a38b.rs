use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let value: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });

    println!("Waiting on thread");

    match value.join() {
        Ok(v) => println!("value: {}", v),
        Err(e) => println!("error joining thread: {:?}", e),
    }
}
