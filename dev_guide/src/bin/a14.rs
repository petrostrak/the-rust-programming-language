// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: u32,
    name: String,
    favorite_color: String,
}

impl Person {
    fn print_color(&self) {
        println!("{}", self.favorite_color)
    }

    fn print_name(&self) {
        println!("{}", self.name)
    }
}

fn main() {
    let pit: Person = Person {
        age: 36,
        name: String::from("Pit"),
        favorite_color: "green".to_string(),
    };
    let axl: Person = Person {
        age: 45,
        name: String::from("Axl"),
        favorite_color: "Red".to_string(),
    };
    let mag: Person = Person {
        age: 38,
        name: String::from("Maggie"),
        favorite_color: "Pink".to_string(),
    };
    let geo: Person = Person {
        age: 9,
        name: String::from("George"),
        favorite_color: "Blue".to_string(),
    };

    let persons: Vec<Person> = vec![pit, axl, mag, geo];

    for person in persons.iter() {
        if person.age < 10 {
            person.print_name();
            person.print_color();
        }
    }
}
