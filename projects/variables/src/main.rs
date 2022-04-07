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

    // The Array Type
    let arr = [1, 2, 3, 4, 5];

    let same_values = [3; 5]; // [3, 3, 3, 3, 3]

    let second_arr = [1, 2, 3, 4, 5];
    let first_element = second_arr[0]; // 1

    // Invalid Array Element Access
    let mut index = String::new();

    std::io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse()
        .expect("Not a number");
    
    let element = arr[index];

    println!("The value of the element at index {} is {}.", index, element);
}
