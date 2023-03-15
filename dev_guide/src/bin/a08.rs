// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn get_drink_info(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => print!("Sparkling, "),
        Flavor::Sweet => print!("Sweet, "),
        Flavor::Fruity => print!("Fruity, "),
    }
    println!("with {} ounces.", drink.fluid_oz)
}

fn main() {
    let zombie: Drink = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 2.5,
    };
    let spritz: Drink = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 1.5,
    };
    let beer: Drink = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 1.0,
    };

    let cellar: Vec<Drink> = vec![zombie, spritz, beer];

    for drink in cellar {
        get_drink_info(drink)
    }
}
