// Slices let you reference a contiguous sequence of elements in a collection rather
// than the whole collection.
fn main() {
    let s = String::from("Hello world!");
    let word = first_word(&s[..]);
    // s.clear();
    println!("{}", word)

    // let hello = &s[..5];
    // let world = &s[6..];
    // println!("{} {}", hello, world);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item ==  b' ' {
            return &s[..i];
        }
    }

    &s[..]
}