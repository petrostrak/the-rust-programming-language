use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // guess is mutable therefore it can be assigned
    // to different values.
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess)
}
