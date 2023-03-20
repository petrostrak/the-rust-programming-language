// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn perimeter(&self) -> f32;
}

struct Square {
    side: f32,
}
impl Perimeter for Square {
    fn perimeter(&self) -> f32 {
        self.side * 4 as f32
    }
}

struct Triangle {
    base: f32,
    side_1: f32,
    side_2: f32,
}
impl Perimeter for Triangle {
    fn perimeter(&self) -> f32 {
        self.base + self.side_1 + self.side_2
    }
}

fn print_perimeter(shape: impl Perimeter) {
    println!("{:.2}", shape.perimeter())
}

fn main() {
    let square: Square = Square { side: 5.5 };
    print_perimeter(square);

    let triangle: Triangle = Triangle {
        base: 3.0,
        side_1: 6.1,
        side_2: 6.1,
    };
    print_perimeter(triangle);
}
