fn main() {
    let string1 = String::from("abcd");
    let string2 = String:: from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}

// &i32         a reference
// &'a i32      a reference with an explicit lifetime
// &'a mut i32  a mutable reference with an explicit lifetime 
//
// Generical lifetime annotations create a relation between
// the arguments lifetime and the returned reference.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // the lifetime of the returned reference will 
    if x.len() > y.len() {                          // be the same as the smallest lifetime of the 
        x                                           // arguments lifetime.
    } else {
        y
    }
}

struct ImportantExcerpt<'a> { // the struct cannot outlive the reference passed into part.
    part: &'a str,
}