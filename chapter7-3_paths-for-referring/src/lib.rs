// pub fn eat_at_restaurant() {
//     create::front_of_house::hosting::add_to_waitlist();

//     front_of_house::hosting::add_to_waitlist();
// }

// fn deliver_order() {}

// Making Structs and Enums Public
// mod back_of_house() {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// }

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order 1= back_of_house::Appetizer::Soup;
    let order 2 = back_of_house::Appetizer::Salad;
}