// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Box {
    dimensions: Dimensions,
    weight: i32,
    color: Color,
}

impl Box {
    fn new(dimensions: Dimensions, weight: i32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_box(&self) {
        self.dimensions.print_dimensions();
        self.color.print_color();
        println!("Weight: {}", self.weight);
    }
}

struct Dimensions {
    height: i32,
    width: i32,
    length: i32,
}

impl Dimensions {
    fn print_dimensions(&self) {
        println!(
            "Height: {}, Width: {}, Length: {}",
            self.height, self.width, self.length
        )
    }
}

enum Color {
    Yellow,
    Black,
    White,
}

impl Color {
    fn print_color(&self) {
        match self {
            Color::Yellow => println!("Yellow box"),
            Color::Black => println!("Black box"),
            Color::White => println!("White box"),
        }
    }
}

fn main() {
    let dimensions = Dimensions {
        height: 45,
        width: 30,
        length: 60,
    };
    let color: Color = Color::Yellow;
    let package: Box = Box::new(dimensions, 8, color);
    package.print_box();
}
