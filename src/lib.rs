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
    #[ignore]
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

    use super::*;

    #[test]
    fn one_ok_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            mini_grep::search(query, contents)
        );
    }

    #[test]
    fn one_failed_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_ne!(
            vec!["not contains this string"],
            mini_grep::search(query, contents)
        );
    }
}

// run following command to test mini_grep::search
// cargo test --package hello_cargo --lib tests -- --test-threads=1 --nocapture cargo
pub mod mini_grep {
    use std::{env, fs, process, io, error::Error};

    pub struct Config {
        pub query: String,
        pub filename: String,
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;

        println!("file content is :\n{}\n", &contents);

        let lines: Vec<&str> = search(&config.query, &contents);

        println!("there are {} matched lines:", lines.len());

        for line in lines {
            println!("> {}", line);
        }
        Ok(())
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            Ok(Config { query: args[1].clone(), filename: args[2].clone() })
        }
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        results
    }
}
