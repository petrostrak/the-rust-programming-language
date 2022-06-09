pub trait Iterator {
    type Item;

    fn  next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // v1_iter must be mutable because we are
    // gonna call .next() which needs a mutable
    // reference to self, aka that iterator.
    // 
    // .iter() returns immutable references.
    // let mut v1_iter = v1.iter();

    // .iter_mut() returns mutable references.
    // let mut v1_iter = v1.iter_mut(); 

    // .into_iter() returns owned types.
    let mut v1_iter = v1.into_iter(); 

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }
}