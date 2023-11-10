#![allow(unused)]

use crate::front_of_house::hosting::walk_to_table;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn walk_to_table() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}        
    }
}


fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() { 
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // Can be private at the field level
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // All or nothing
    pub enum Appetizer {
        Soup,
        Salad,
    }    

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }    
}

pub fn eat_at_restaurant() {
    // import multiple ways
    // crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();
    // use crate::front_of_house::hosting::add_to_waitlist;

    // Bring in Appetizer and Braeakfast
    use back_of_house::{Appetizer, Breakfast};

    // Bring in hosting and hosting::add_to_waitlist
    use front_of_house::hosting::{self, add_to_waitlist};

    add_to_waitlist();
    hosting::walk_to_table();

    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);    

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;    
}