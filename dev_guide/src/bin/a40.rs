// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Rental {
    type_of_vehicle: Vehicle,
    vin: String,
    vehicle_status: VehicleStatus,
}

#[derive(Debug)]
enum Vehicle {
    Car,
    Truck,
}

#[derive(Debug, Hash, PartialEq, PartialOrd)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct Corporate(Rent);

#[derive(Debug)]
struct StoreFront(Rent);

// RefCell can mutate its content. Being in an Rc, means that every
// Rent variable can mutate the contents of the Rental List.
type Rent = Rc<RefCell<Vec<Rental>>>;

fn new_rental() -> Rental {
    Rental {
        type_of_vehicle: Vehicle::Car,
        vin: "947653".to_owned(),
        vehicle_status: VehicleStatus::Rented,
    }
}

fn main() {
    let rentals: Rent = Rc::new(RefCell::new(vec![]));
    let rental = new_rental();
    let corp = Corporate(Rc::clone(&rentals));
    let store_front = StoreFront(Rc::clone(&rentals));

    {
        rentals.borrow_mut().push(rental);
    }
    dbg!(corp.0.borrow());
    dbg!(store_front.0.borrow());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_status() {
        let vehicles = vec![
            Rental {
                vehicle_status: VehicleStatus::Available,
                type_of_vehicle: Vehicle::Car,
                vin: "123".to_owned(),
            },
            Rental {
                vehicle_status: VehicleStatus::Maintenance,
                type_of_vehicle: Vehicle::Truck,
                vin: "abcd".to_owned(),
            },
        ];

        // With RefCell we can modify the contents, with Rc we can share em.
        let vehicles = Rc::new(RefCell::new(vehicles));

        // When we are cloning via Rc (Rc::clone()), we are not cloning any data,
        // we are cloning the pointer itself.
        let corp = Corporate(Rc::clone(&vehicles));
        let store = StoreFront(Rc::clone(&vehicles));

        {
            let mut rentals = store.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.vehicle_status, VehicleStatus::Available);
                car.vehicle_status = VehicleStatus::Rented;
            }
        }

        {
            let mut rentals = corp.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.vehicle_status, VehicleStatus::Rented);
                car.vehicle_status = VehicleStatus::Available;
            }
        }

        let rentals = store.0.borrow();
        if let Some(car) = rentals.get(0) {
            assert_eq!(car.vehicle_status, VehicleStatus::Available);
        }
    }
}
