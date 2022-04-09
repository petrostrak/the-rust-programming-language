fn main() {
    println!("Hello, world!");

    another_function();
    func_with_params(5, 4);

    let x = five();
    println!("The value of x is {}.", x);

    let z = plus_one(5);
    println!("The value of z is {}.", z);
}

fn another_function() {
    println!("Another function.");
}

fn func_with_params(x: i32, y: i32) {
    println!("The value of x is {} and y {}.", x, y);
}

fn five() -> i32 {
    5 // adding a semicoln (;) aka from an expression to a statement, we are getting an error
}

fn plus_one(x: i32) -> i32 {
    x + 1
}