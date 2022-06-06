fn main() {
    let number_list = vec![34, 50, 100, 65];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = get_largest(char_list);

    println!("The largest number is {}", largest);

    let p1 = Point{x:5, y:10};
    let p2 = Point{x: 1.5, y: 3.3};
    let p3 = Point{x: 1.5, y: 3};
}

struct Point<T, U> {
    x: T,
    y: U,
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