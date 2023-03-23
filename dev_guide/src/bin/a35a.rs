enum Status {
    Error(i32),
    Info,
    Warn,
}

fn main() {
    let status = Status::Error(5);
    match status {
        // @ is called binding
        Status::Error(s @ 3) => println!("error #3"),
        Status::Error(s @ 5..=6) => println!("error #5 or #6"),
        Status::Error(s @ 4..=10) => println!("error #4 through #10"),
        Status::Error(s @ 18 | s @ 19) => println!("error #18 or #19"),
        Status::Error(s) => println!("error code #{}", s),
        Status::Info => println!("info"),
        Status::Warn => println!("warning"),
    }
}
