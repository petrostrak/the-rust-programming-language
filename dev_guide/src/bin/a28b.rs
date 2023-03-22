// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct Shoes(Color);
impl Shoes {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

#[derive(Debug)]
struct Shirt(Color);
impl Shirt {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

#[derive(Debug)]
struct Pants(Color);
impl Pants {
    fn new(c: Color) -> Self {
        Self(c)
    }
}

fn print_shoes_color(c: Shoes) {
    println!("shoes color: {:?}", c)
}

fn print_shirt_color(c: Shirt) {
    println!("shirt color: {:?}", c)
}

fn print_pants_color(c: Pants) {
    println!("pants color: {:?}", c)
}

fn main() {
    let white_shoes = Shoes(Color::White);
    let black_shirt = Shirt(Color::Black);
    let navy_pants = Pants(Color::Custom("Navy".to_owned()));

    print_shoes_color(white_shoes);
    print_shirt_color(black_shirt);
    print_pants_color(navy_pants);
}
