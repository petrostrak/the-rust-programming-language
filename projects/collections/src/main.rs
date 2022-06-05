use unicode_segmentation::{self, UnicodeSegmentation};
use std::collections::HashMap;

fn main() {
    let a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

   { let v2 = vec![1, 2, 3];}

   let third = &v[2]; // immutable reference to value of vector
//    v.push(6); // mutable reference to push to vector. We expect the underlying value not to change
   println!("The third element is {}", third); // we use the immutable reference.

   match v.get(2) {
       Some(third) => println!("The third element is {}", third),
       None => println!("There is no third element"),
   }

   let mut v3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

   for i in &v3 {
       println!("{}", i)
   }

   for y in &mut v3 {
    *y += 50
   }

   for i in &v3 {
    println!("{}", i);
   }

   enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }   

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer")
    }

    // Strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "initial contents";
    let s3  = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    // foobar!

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3 = s1 + &s2; 
    let s3 = format!("{}{}", s1, s2);

    let hello = String::from("今日は");

    for b in hello.bytes() {
        print!("{}\n", b);
    }

    for c in hello.chars() {
        println!("{}", c);
    }

    for g in hello.graphemes(true) {
        println!("{}", g);
    }

    // Hashmaps
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}",score);

}

