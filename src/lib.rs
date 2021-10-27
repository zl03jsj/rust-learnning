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

    use super::mini_grep::{search};

    #[test]
    fn test_search_sensitive() {
        let (small_start_query, big_start_query) = ("duct", "Duct");
        let case_sensitive = true;
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_ne!(vec!["safe, fast, productive."], search(big_start_query, contents, case_sensitive));

        assert_eq!(vec!["safe, fast, productive."], search(big_start_query, contents, !case_sensitive));

        assert_eq!(vec!["safe, fast, productive."], search(small_start_query, contents, case_sensitive));

        assert_eq!(vec!["safe, fast, productive."], search(small_start_query, contents, !case_sensitive));

        assert_ne!(vec!["safe, fast, productive."], search("not contains this string", contents, case_sensitive));
    }
}

// run following command to test mini_grep::search
// cargo test --package hello_cargo --lib tests -- --test-threads=1 --nocapture cargo
pub mod mini_grep {
    use std::{env, fs, process, io, error::Error};

    pub struct Config {
        pub query: String,
        pub filename: String,
        pub case_sensitive: bool,
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;

        println!("file content is :\n{}\n", &contents);

        let lines: Vec<&str> = search(&config.query, &contents, config.case_sensitive);

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
            Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
            })
        }
    }

    pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
        let mut results = Vec::new();
        for line in contents.lines() {
            if (case_sensitive && line.contains(query)) ||
                (!case_sensitive && line.to_lowercase().contains(query.to_lowercase().as_str())) {
                results.push(line)
            }
        }
        results
    }
}
