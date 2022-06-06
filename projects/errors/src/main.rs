use std::fs::File;

fn main() {
    // a();

    let f = File::open("hello.txt");

    let f = match  f {
        Ok(file) => file,
        Err(error) => panic!("problem opening the file: {:?}", error),
    };
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("don't pass in 22!")
    }
}