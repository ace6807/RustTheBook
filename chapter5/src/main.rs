#![allow(dead_code)]

mod test;
mod rectangle;
mod methods;

fn main() {
    let mut user1 = build_user();
    println!("{}", user1.email);
    let u2: User = User{email: String::from("Test"), username: String::from("Another"), ..user1};
    set_email(&mut user1);
    println!("{}", user1.email);
    println!("{}", u2.email);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("{:?}", black);
    println!("{:?}", origin);

    println!("{}", black.0);

    rectangle::main();
    methods::main();
}

fn set_email(u: &mut User) {
    u.email = String::from("Foo@bar.com");
}

fn build_user() -> User{
    let user1 = User{
        active: true,
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        sign_in_count: 1,
    };

    return user1;

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

