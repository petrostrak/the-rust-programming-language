struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("PetrosTrak"),
        active: false,
        sign_in_count: 0,
    };

    let mut user2 = User {
        username: String::from("alx"),
        email: String::from("alx@example.com"),
        ..user1
    };
    user2.email = String::from("new_email@example.com");

    let user3 = build_user(
        String::from("Maggie"), 
        String::from("mag@example.com"), 
        20, 
        user1.active,
    );

}

fn build_user(username: String, email: String, counter: u64, active: bool) -> User {
    User {
        username,
        email,
        active,
        sign_in_count: counter,
    }
}