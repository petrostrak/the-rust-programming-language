// Shared Ownership - Smart pointers

use std::rc::Rc;

#[derive(Debug)]
struct Vehicle {
    vin: String,
}

#[derive(Debug)]
struct Door {
    vehicle: Rc<Vehicle>,
}

fn main() {
    let car = Rc::new(Vehicle {
        vin: "123456789".to_owned(),
    });

    let left_door = Door {
        vehicle: Rc::clone(&car), // Rc creates a new owner, therefore when car is droped, it persists
    };
    let right_door = Door {
        vehicle: Rc::clone(&car), // Rc creates a new owner, therefore when car is droped, it persists
    };
    drop(car);
    println!("{:?}", left_door.vehicle);
    println!("{}", right_door.vehicle.vin);
}
