// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// // pub fn eat_at_restaurant() {
// //     hosting::add_to_waitlist();
// // }

// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

//Creating Idiomatic use Paths
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// use std::io;
// use std::io::Write;
// use std::io::{self, Write};
// use std::collections::*;
