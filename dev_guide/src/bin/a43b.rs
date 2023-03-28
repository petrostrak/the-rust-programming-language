// Index Operator
// It is possible to change how indexing works in Rust
// by implementing the Index trait.
use std::ops::Index;

enum Temp {
    Current,
    Max,
    Min,
}

struct Hvac {
    current_temp: i16,
    max_temp: i16,
    min_temp: i16,
}

impl Index<Temp> for Hvac {
    type Output = i16;
    fn index(&self, index: Temp) -> &Self::Output {
        match index {
            Temp::Current => &self.current_temp,
            Temp::Max => &self.max_temp,
            Temp::Min => &self.min_temp,
        }
    }
}

fn main() {
    let env = Hvac {
        current_temp: 30,
        max_temp: 60,
        min_temp: 0,
    };

    let current = env[Temp::Current];
}
