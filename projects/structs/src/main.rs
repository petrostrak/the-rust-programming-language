fn main() {

    let Petros = User{
        email: String::from("someone@example.com"),
        username: String::from("PetrosTrak"),
        active: false,
        sign_in_count: 0,
    };

    fn build_user(username: String, email: String, counter: u64, active: bool) -> User {
        User {
            username,
            email,
            active,
            sign_in_count: counter,
        }
    }

    let Alex = User {
        username: String::from("alx"),
        email: String::from("alx@example.com"),
        ..Petros
    };

    let Maggie = build_user("Maggie".to_string(), "mag@example.com".to_string(), 20, false);

}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}