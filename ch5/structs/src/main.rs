
struct User {
        active:bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


struct AlwaysEqual;


fn main() {
    let mut user1 = User{
        active: true,
        username: String::from("uNama"),
        email: String::from("uNama@gmail.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let user4 = build_user("uname2@gmail.com".to_string(), "uname2".to_string()); 
    user1.email = String::from("realEmail@gmail.com");

    println!("{}", user1.email);
    println!("{}", user4.email);


    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
    

}

fn build_user(email: String, username: String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}