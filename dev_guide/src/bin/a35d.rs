struct Vehicle {
    km: usize,
    year: usize,
}

fn main() {
    let car = Vehicle {
        km: 80_000,
        year: 2023,
    };

    match car {
        Vehicle { km, year } if km == 0 && year == 2023 => println!("Brand new Bj 2020"),
        Vehicle { km, .. } if km <= 50_000 => println!("Used under 50k km"),
        Vehicle { km, .. } if km >= 80_000 => println!("Used at least 80k km"),
        Vehicle { year, .. } if year == 2023 => println!("Made in 2023"),
        Vehicle { .. } => println!("Other mileage"),
    }
}
