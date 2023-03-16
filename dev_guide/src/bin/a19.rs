// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut store: HashMap<String, u16> = HashMap::new();
    store.insert(String::from("chair"), 5);
    store.insert(String::from("bed"), 3);
    store.insert(String::from("table"), 2);
    store.insert(String::from("couch"), 0);

    let mut total_items: u16 = 0;
    for (furniture, quantity) in store.iter() {
        let stock = if *quantity == 0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", quantity)
        };
        println!("{}: {}", furniture, stock);
        total_items += quantity;
    }
    println!("Total items: {}", total_items);
}
