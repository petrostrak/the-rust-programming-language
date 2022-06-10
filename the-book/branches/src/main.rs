fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = false;
    let number = if condition {
        4
    } else {
        // "five" the values that have the potential to be results from each arm
        // of the if must be the same type. Variables must have a single type.
        5
    };

    println!("The valiue of number is: {}", number);

}
