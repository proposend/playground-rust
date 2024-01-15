// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct AlwaysEqual;

fn main() {1
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("foo"),
    //     email: String::from("foo@example.com"),
    //     sign_in_count: 1,
    // };

    // user1.email = String::from("bar@example.com");
    // println!("user1 {:?}", user1);

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("bar@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // println!("user2 {:?}", user2);

    // {
    //     let user3 = User {
    //         email: String::from("foobar@example.com"),
    //         ..user1
    //     };
    //     println!("user3 {:?}", user3);
    // }
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black);
    println!("{:?}", origin);

    let subject = AlwaysEqual;
    println!("{:?}", subject);
}

// fn builder_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username, // username: username, // shorthand
//         email,    // email: email, // shorthand
//         sign_in_count: 1,
//     }
// }
