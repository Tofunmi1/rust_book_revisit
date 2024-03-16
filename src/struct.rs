struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/// @note this wont work , storing references is in chapter 10
// struct User01 {
//     active : bool,
//     username: &str,
//     email:&str,
//     sign_in_count : u64,
// }

fn main() {
    let mut user1: User = User {
        active: true,
        username: String::from(""),
        email: String::from(""),
        sign_in_count: 1,
    };

    let mut user2: User = User {
        active: true,
        ..user1
    };
    user1.email = String::from("adediran@gmail.com");

    let black: Color = Color(3, 1, 2);
    let x: i32 = black.0;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
