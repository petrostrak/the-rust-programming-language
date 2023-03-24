#[derive(Debug)]
struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(value: String) -> Self {
        Self(value.to_uppercase())
    }
}

impl From<&str> for Uppercase {
    fn from(value: &str) -> Self {
        Self(value.to_uppercase())
    }
}

fn main() {
    let upper = Uppercase::from("pit trak");
    println!("{:?}", upper);
    let upper: Uppercase = "pit trak".into();
    println!("{:?}", upper);
}
