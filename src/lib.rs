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
pub(crate) mod mini_grep;

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

        r#enum::test_call();
        r#enum::test_ip();
        r#struct::rectangle_area();
        r#match::test_match();
        restaurant::run::eat_at_restaurant();
    }

    use crate::mini_grep::{search};

    // run following command to test mini_grep::search
    // cargo test --package hello_cargo --lib tests -- --test-threads=1 --nocapture cargo
    #[test]
    #[ignore]
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

    #[test]
    fn play_game() {
        game::start();
    }
}

// #[ignore]标记, 默认测试的时候, 测试会被忽略.
// 需要在命令行指定: `cargo --ignore` 才会测试`#[ignore]`标记的单元测试.
// cargo test --package hello_cargo --lib tests -- --test-threads=1 --nocapture cargo --ignored

// 指定测试项, 例如: play_game, 由于game需要通过命令行来交互, 需要显示命令行输出, 所以指定了`nocapture`.
// cargo test --package hello_cargo --lib tests::play_game -- --nocapture
