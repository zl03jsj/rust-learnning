use super::backend::Food;

pub mod hosting {
    use crate::restaurant::backend::Food;

    pub fn add_to_waitlist(food: &Food) {
        println!("add food{:?} to wailtlist", food)
    }
}
