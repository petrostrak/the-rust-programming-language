// Fibonacci iterator

struct F {
    number: (usize, usize),
}
impl F {
    fn new() -> Self {
        Self { number: (0, 1) }
    }
}

impl Iterator for F {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.number = (self.number.1, self.number.0 + self.number.1);
        Some(self.number.0)
    }
}

fn main() {
    let fib = F::new().take(10);
    for f in fib {
        println!("{:?}", f)
    }
}
