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
    println!("{}", i)
}
}
