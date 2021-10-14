#[derive(Debug)]
pub enum Food {
    Breakfast(Breakfast),
    Lunch(Lunch),
    Dinner(Dinner),
}

#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

#[derive(Debug)]
pub struct Lunch {}

#[derive(Debug)]
pub struct Dinner {}

impl Lunch {
    pub fn summer() -> Lunch {
        Lunch {}
    }
}

impl Dinner {
    pub fn summer() -> Dinner {
        Dinner {}
    }
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}

pub fn serve_order() {}


pub fn cook_order() {}

pub fn fix_incorrect_order() {
    cook_order();
    serve_order();
}
