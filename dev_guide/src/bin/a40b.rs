// Cell
use std::cell::Cell;

#[derive(Debug)]
struct Book {
    signed: Cell<bool>, // Cell is always mutable
}

impl Book {
    fn sign(&self) {
        self.signed.set(true);
    }
    fn signed(&self) -> bool {
        self.signed.get()
    }
}

fn main() {
    let my_book = Book {
        signed: Cell::new(false),
    };

    println!("signed: {}", my_book.signed());
    my_book.sign();
    println!("signed: {}", my_book.signed());
}
