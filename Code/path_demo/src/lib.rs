// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();

//     front_of_house::hosting::add_to_waitlist();
// }


fn serve_order() {}

// mod back_of_house {

//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//         crate::serve_order();
//     }

//     fn cook_order() {}
// }


// mod back_of_house {
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
//     println!("I' d like {} toast please", meal.toast);
//     meal.seasonal_fruit = String::from("blueberries"); // 无法访问私有字段
// }


// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }



mod front_of_house;

use std::collections::HashMap;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // hosting::some_function();
}

