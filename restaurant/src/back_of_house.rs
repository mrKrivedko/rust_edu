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
