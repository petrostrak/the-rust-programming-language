fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // let y = s;                   // compile-time error. Value s used after move
                                
    let x = 5;

    makes_copy(x);                  // x would move into the function, but i32
                                    // is Copy, so it's ok to still use x afterwards.
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string)
} // here, some_string goes out of scope and `drop` is called. The backing memory is free.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer)
} // here, some_integer goes out of scope. Nothing special happens.