enum Species {
    Finch,
    Hawk,
    Parrot,
}

struct Bird {
    age: usize,
    species: Species,
}

fn main() {
    let hawk = Bird {
        age: 13,
        species: Species::Hawk,
    };

    match hawk {
        Bird { age: 4, .. } => println!("4 year old bird"), // we match on this struct only if age is 4
        Bird {
            age: 4..=10 | 15..=20,
            ..
        } => println!("4-10 or 15-20 years old"),
        Bird {
            species: Species::Finch,
            ..
        } => println!("finch"), // we match on this on only if its Species::Finch
        Bird { .. } => println!("other bird"), // match on everything else
    }
}
