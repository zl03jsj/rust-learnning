#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_code)]

mod guess;
mod syntax;
mod restaurant;
mod dtd_types;
mod exception;

use crate::{
    guess::game,
    syntax::{r#struct, r#enum, r#match},
    dtd_types::{vector, string, hash_map},
};

fn main() {
    exception::read_username_from_file(None);
    exception::test_exception(false);
    return;
    exception::test_unwarp_or_else();
    hash_map::test_map();
    string::test_string();
    vector::test_vector();
    game::start();
    r#enum::test_call();
    r#enum::test_ip();
    r#struct::rectangle_area();
    r#match::test_match();
    restaurant::run::eat_at_restaurant();
}
