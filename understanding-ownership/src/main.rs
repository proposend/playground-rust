fn main() {
    // The main purpose of ownership is to manage heap data
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // The String Type
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // print!("{}", s);

    // let s1 = String::from("hello");
    // let s1 = "hello";
    // let s2 = s1;

    // println!("{} world!", s1);
    // println!("{} world!", s2);

    // Variables and Data Interacting with Clone
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: Copy
    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // Ownership and Functions
    // let s = String::from("hello");
    // takes_ownership(s);
    // println!("{}", s);
    // let x = 5;
    // makes_copy(x);
    // println!("{}", x);

    // Return Values and Scope
    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // println!("{},{}", s1, s3);

    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    // 4.2 References and Borrowing
    // let s1 = String::from("hello");
    // let len = calculate_length2(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // Mutable References
    // 가변 참조자와 불변참조자
    // 가변 참조자는 동시에 두개 이상 있을수없음. 스코프를 만들어 여러개 존재하도록 가능
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", s);

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // println!("{}, ", r1);
    // {
    //     let r2 = &mut s;
    //     println!("{}", r2);
    // }
    // let r3 = &mut s;
    // println!("{}", r3);

    // let r1 = &s;
    // let r2 = &s;

    // println!("{} and {}", r1, r2);
    // let r4 = r1;
    // println!("{}", r4);

    // Dangling References
    let reference_to_nothing = dangle();
}

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// fn calculate_length2(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn dangle() -> String {
    let s = String::from("hello");
    s
}
