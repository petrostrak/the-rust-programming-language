fn main() {
    let s = String::from("hello");

    let len = calculate_length(&s); // The ampersand is reference and the allow to refer to some
                                    // value without taking the ownership.

    println!("The length of '{}' is {}", s, len)
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership of
    // what it refers to, nothing happens.