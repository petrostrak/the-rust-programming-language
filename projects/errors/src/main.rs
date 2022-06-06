use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // a();

    let f = File::open("hello.txt").expect("failed to open hello.txt");
    // let f = File::open("hello.txt").unwrap();

    // let f = match  f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("problem creating the file: {:?}", e),
    //         }
    //         other_error => {
    //             panic!("problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };
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