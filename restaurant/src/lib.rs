use std::fmt::Result;
use std::io::Result as IoResult;

use std::cmp::Ordering;
use std::io;

// можно написать так
// use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;

// можно написать так
// use std::io::{self, Write};

use std::collections::*;  // оператор * подключает всё из модуля в область видимости





mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        
        fn serve_orders() {}

        fn take_payments() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,  // в структурах полям нужно задавать доступ
        season_fruit: String
    }

    pub enum Appetizier {  // в enum при pub сразу доступны все перечисления
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrectorder() {
        cook_order();
        super::deliver_order();  // super похоже на .. в path
    }

    fn cook_order() {}
}

pub use crate::front_of_house::hosting;  // теперь для внешних модулей можно использовать путь restaurant::hosting::add_to_waitlist

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();;

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizier::Soup;
    let order2 = back_of_house::Appetizier::Salad;
}

pub fn new_eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast)

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

/*
Дерево модулей

crate
 └── front_of_house
 ├── hosting
 │   ├── add_to_waitlist
 │   └── seat_at_table
 └── serving
     ├── take_order
     ├── serve_order
     └── take_payment
 */
