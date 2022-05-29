#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("PetrosTrak"),
        active: false,
        sign_in_count: 0,
    };
    println!("User 1: {:?}", user1);

    let mut user2 = User {
        username: String::from("alx"),
        email: String::from("alx@example.com"),
        ..user1
    };
    user2.email = String::from("new_email@example.com");
    println!("User 2: {:?}", &user2);

    let user3 = build_user(
        String::from("Maggie"), 
        String::from("mag@example.com"), 
        20, 
        user1.active,
    );
    println!("User 3: {:?}", &user3);

    let rect= Rectangle{
        height: 30,
        width: 50,
    };
    println!("The rectangle object: {:?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect),
    )


}

fn build_user(username: String, email: String, counter: u64, active: bool) -> User {
    User {
        username,
        email,
        active,
        sign_in_count: counter,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}