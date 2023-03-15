// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn is_bigger(value: i8) -> bool {
    if value > 100 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let value: i8 = 127;
    match is_bigger(value) {
        true => println!("its big"),
        false => println!("its small"),
    }
}
