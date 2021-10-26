#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub(crate) mod guess;
pub(crate) mod syntax;
pub(crate) mod restaurant;
pub(crate) mod exception;
pub(crate) mod generics;
pub(crate) mod dtd_types;

mod tests {
    use std::fmt::Debug;
    use std::io::Error;
    use crate::{exception, generics, syntax, restaurant,
                guess::game,
                syntax::{r#struct, r#enum, r#match},
                dtd_types::{vector, string, hash_map},
    };

    #[test]
    #[should_panic]
    #[ignore]
    fn test_add() {
        assert_eq!(3.2, generics::add(1.1, 2.2), "1.0 add 2.1 should equals 3.1");
    }

    #[test]
    fn run() {
        assert_eq!(3, generics::add(1, 2));
        assert_eq!(3.1, generics::add(1.0, 2.1), "1.0 add 2.1 should equals 3.1");

        generics::test_number_add();
        generics::test_lifetime();

        generics::test_larget_number();
        generics::test_point();
        generics::r#trait::test_trait();

        match exception::read_username_from_file(None) {
            Ok(s) => println!("ok"),
            Err(e) => println!("error:{:?}", e),
        }

        exception::test_exception(false);

        exception::test_unwarp_or_else();
        hash_map::test_map();
        string::test_string();
        vector::test_vector();

        // game::start();

        r#enum::test_call();
        r#enum::test_ip();
        r#struct::rectangle_area();
        r#match::test_match();
        restaurant::run::eat_at_restaurant();
    }
}
