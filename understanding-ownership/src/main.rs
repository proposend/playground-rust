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
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
