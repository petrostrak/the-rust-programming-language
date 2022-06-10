fn main() {
    let number_list = vec![34, 50, 100, 65];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = get_largest(char_list);

    println!("The largest number is {}", largest);

    let p1 = Point{x:5, y:10};
    let p2 = Point1{x: 1.5, y: 3.3};
    let p = Point1{x: "Hello", y: 'c'};
    p1.x();
    let p3 = p2.mixup(p);
}

struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 { x: self.x, y: other.y }
    }
}

struct Point<U> {
    x: U,
    y: U,
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest_char(number_list: Vec<char>) -> char {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}