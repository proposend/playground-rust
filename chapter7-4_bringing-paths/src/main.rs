use std::collections::HashMap;
// use std::{fmt, io};
// use std::fmt::Result;
// use std::io::Result as IoResult;
use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

fn main() {
    // let mut map = HashMap::new();
    // map.insert(1, 2);
    // println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number : {}", secret_number);
}

// fn function1() -> fmt::Result {}

// fn function2() -> io::Result<()> {}

// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}
