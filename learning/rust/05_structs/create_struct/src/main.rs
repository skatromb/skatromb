struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
        );
    
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
    
    let user2 = User {
        email: String::from("yetanotheremail@yandex.ru"),
        ..user1
    };
    
    println!("{}", user1.email);
    println!("{}", user2.email);
}

