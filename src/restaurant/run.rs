#![allow(unused_mut)]

use super::front::hosting;
use super::backend;
use crate::restaurant::backend::Food;

fn print_food(food: &Food) {
    match food {
        Food::Breakfast(breakfast) => println!("i will eat food:{:?}", breakfast),
        Food::Dinner(dinner) => println!("i will eat food:{:?}", dinner),
        Food::Lunch(lunch) => println!("i will eat food:{:?}", lunch),
        _ => println!("i don't know what to eat!"),
    }
}

pub fn eat_at_restaurant() {
    let mut meal = backend::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let food = Food::Breakfast(meal);

    print_food(&food);

    hosting::add_to_waitlist(&food);
    backend::cook_order();
    backend::serve_order();
}
