// Serde

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    email: String,
    age: u8,
}

fn main() {
    let form = Person {
        email: "pit.trak@gmail.com".to_owned(),
        name: "pit".to_owned(),
        age: 36,
    };

    let serialized_data = serde_json::to_string(&form).expect("failed to serialize");
    println!("{}", serialized_data);

    let deserialized_data = serde_json::from_str::<Person>(&serialized_data);
    println!("{:?}", deserialized_data.unwrap());
}
