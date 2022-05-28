fn main() {
    // ------ Ownership rules ------
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valud here, it's not yet declared
        let s = String::from("Hello!"); // s is valid from this point onwards
        // do stuff with s
        takes_ownership(s);
        // println!("{}", s); // value borrowed here after move

        let s1 = gives_ownership();
        println!("{}", s1);
    } // this scope is now over, and s is no longer valid

    let s = String::from("hello");
    let length = calculate_length(&s); // s points to line 17 s
    println!("The length of '{}' is {}.", s, length);

    let mut s1 = String::from("Hello");
    change(&mut s1);
    println!("{}", s1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");

    some_string
}

// Reference does not take ownership of the underlying value.
// Passing references in functions parameters is called borrowing.
// References are immutalble and we cannot modify them.
fn calculate_length(some_string: &String) -> usize {
    // some_string is a pointer to s parameter in line 18.
    some_string.len()
}

// In order to mutate a reference to a string, it has to be a mut variable.
fn change(some_string: &mut String) {
    some_string.push_str(" world!");
}