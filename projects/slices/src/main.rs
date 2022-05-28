fn main() {
    let s = String::from("Hello world!");
    first_word(&s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item ==  b' ' {
            return i;
        }
    }

    s.len()
}