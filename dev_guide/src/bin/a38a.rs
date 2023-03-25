use std::thread;

fn main() {
    let iterations = 10;
    let a = thread::spawn(move || {
        for i in 1..=iterations {
            println!("a:{}", i)
        }
    });

    let b = thread::spawn(move || {
        for i in 1..=iterations {
            println!("  b:{}", i)
        }
    });
    a.join().unwrap();
    b.join().unwrap();
}
