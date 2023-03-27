// Manually implement PartialOrd
// PartialOrd on structs compares only the first member of the struct.
use std::cmp::Ordering;

#[derive(PartialEq)]
struct User {
    id: u32,
    name: String,
}
impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // if self.name < other.name {
        //     Some(Ordering::Less)
        // } else if self.name > other.name {
        //     Some(Ordering::Greater)
        // } else {
        //     Some(Ordering::Equal)
        // }
        Some(self.name.cmp(&other.name))
    }
}

fn main() {}
