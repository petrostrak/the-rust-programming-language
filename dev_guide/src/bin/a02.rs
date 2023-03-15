// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    println!("{}", add_two(2, 2));
    println!("{}", add_two(2.5, 1.3));
}

fn add_two<T>(a: T, b: T) -> T
where
    T: std::ops::Add<T, Output = T>,
{
    a + b
}

#[cfg(test)]
mod tests {
    use crate::add_two;

    #[test]
    fn add_two_test() {
        let result = 3.8;
        assert_eq!(result, add_two(2.5, 1.3))
    }
}
