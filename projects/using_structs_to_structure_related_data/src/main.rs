fn main() {
    println!("Hello, world!");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active: true,
        username: String::from("arnold_the_jedi"),
        email: String::from("arnoldt100@hotmail.com"),
        sign_in_count: 1,
    };

    println!("User1 email: {}",user1.email);

    user1.email = String::from("arnoldt100@gmail.com");

    println!("User1 email: {}",user1.email);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let email = String::from("arnoldt@ornl.gov");
    let username = String::from("8nt");
    let user2 = build_user(email,username);
    println!("User2 email: {}",user2.email);

    let user3 = User{ email: String::from("8nt@ornl.gov"),
                      ..user1
    };

    println!("User3 email: {}",user3.email);

}
