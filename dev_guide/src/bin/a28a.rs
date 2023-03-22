struct NeverZero(i32);
impl NeverZero {
    fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("cannot devide by zero".to_owned())
        } else {
            Ok(Self(i))
        }
    }
}

fn devide(a: i32, b: NeverZero) -> i32 {
    a / b.0
}

fn main() {
    match NeverZero::new(0) {
        Ok(x) => println!("{:?}", devide(10, x)),
        Err(e) => println!("{:?}", e),
    }
}
