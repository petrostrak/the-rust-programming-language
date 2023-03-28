// Implementing basic Iterator

struct Odd {
    number: isize,
    max: isize,
}
impl Odd {
    fn new(max: isize) -> Self {
        Self { number: -1, max }
    }
}
impl Iterator for Odd {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        self.number += 2;
        if self.number <= self.max {
            Some(self.number)
        } else {
            None
        }
    }
}

fn main() {
    let odds = Odd::new(7);
    for o in odds {
        println!("odd: {}", o)
    }
}
