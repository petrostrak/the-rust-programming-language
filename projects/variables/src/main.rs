fn main() {
    // Shadowing
    let x = 6;
    println!("The value of x is {}", x);
    
    let x = 7;
    let x = x * 3;
    println!("The value of x is {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {}",spaces);

    // Floating-Point Types
    let y = 2.5;
    let z: f32 = 3.5;

    // Boolean Type
    let isNumber = true;
    let isEmpty: bool = false;

    // Compound Types - Tuples
    let tup : (i32, f32, u8) = (500, 1.2, 1);

    let tup = (500, 1.2, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let q: (i32, f32, u8) = (500, 1.2, 1);

    let five_hundrer = q.0;
    let floating_number = q.1;
    let ace = q.2;
}
