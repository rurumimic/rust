mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn show(self: &Breakfast) {
            println!("{} & {}", self.toast, self.seasonal_fruit);
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

// use crate::front_of_house::hosting;
// --> restaurant::front_of_house::hosting::add_to_waitlist()

// re-exporting
pub use crate::front_of_house::hosting;
// --> restaurant::hosting::add_to_waitlist()

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use: like a symbolic link
    hosting::add_to_waitlist();
    add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    //      ^^^^^^^^^^^^^^ private field
    meal.show();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

use std::collections::HashMap;

fn useHashMap() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// use std::fmt::Result;
// use std::io::Result;
//
// the name `Result` is defined multiple times
// `Result` must be defined only once in the type namespace of this module
// help: you can use `as` to change the binding name of the import
// use std::io::Result as OtherResult;

// use std::cmp::Ordering;
// use std::io;
// or
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// or
use std::io::{self, Write};

use std::collections::*;
