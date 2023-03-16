// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: u8,
}

fn is_allowed(person: &Customer) -> Result<(), String> {
    if person.age >= 21 {
        Ok(())
    } else {
        Err("Customer must be 21 years of age or above.".to_owned())
    }
}

fn main() {
    let pit: Customer = Customer { age: 36 };

    let result: Result<(), String> = is_allowed(&pit);
    println!("{:?}", result.unwrap());
}
