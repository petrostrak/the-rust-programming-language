// Operator Overloading
// All overloadable operators are available in std::ops module
use std::ops::Add;

struct Speed(u32);
impl Add<Self> for Speed {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Speed(self.0 + rhs.0)
    }
}

impl Add<u32> for Speed {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Speed(self.0 + rhs)
    }
}

struct Letter(char);
impl Add<Self> for Letter {
    type Output = String;
    fn add(self, rhs: Self) -> Self::Output {
        format!("{}{}", self.0, rhs.0)
    }
}
impl Add<String> for Letter {
    type Output = String;
    fn add(self, rhs: String) -> Self::Output {
        format!("{}{}", self.0, rhs)
    }
}

fn main() {
    let fast = Speed(5) + Speed(5);
    println!("{}", fast.0);

    let faster = fast + 10;
    println!("{}", faster.0);

    let initials: String = Letter('P') + Letter('T');
    println!("{}", initials);
    let full = Letter('g') + initials;
    println!("{}", full)
}
