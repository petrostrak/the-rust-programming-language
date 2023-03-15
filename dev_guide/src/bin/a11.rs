// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    id: i32,
    quantity: i32,
}

fn get_quantity(grocery: &Grocery) {
    println!("Quantity: {}", grocery.quantity)
}

fn get_id(grocery: &Grocery) {
    println!("Id: {}", grocery.id)
}

fn main() {
    let cheese: Grocery = Grocery { id: 7, quantity: 3 };
    get_id(&cheese);
    get_quantity(&cheese);
}
