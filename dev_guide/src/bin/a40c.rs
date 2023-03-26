// RefCell - Returns borrowed data
use std::cell::RefCell;

struct Person {
    name: RefCell<String>,
}

fn main() {
    let name = "Pit".to_owned();
    let person = Person {
        name: RefCell::new(name),
    };

    {
        let mut name = person.name.borrow_mut();
        *name = "Maggie".to_owned();
    }

    {
        person.name.replace("Greg".to_owned());
    }

    println!("name: {:?}", person.name);
}
