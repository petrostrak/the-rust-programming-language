// Advanced Closures

fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let add = |a, b| a + b;
    let add = Box::new(add);
    println!("{}", math(5, 5, add));

    let sub = Box::new(|a, b| a - b);
    println!("{}", math(5, 5, sub));
    let mul = Box::new(|a, b| a * b);
    println!("{}", math(5, 5, mul));
}
