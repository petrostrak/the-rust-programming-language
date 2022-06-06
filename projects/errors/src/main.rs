use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;

fn main() {
    // a();

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return  Err(e)
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    
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