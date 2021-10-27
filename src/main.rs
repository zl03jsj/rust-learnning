#![allow(unused_imports)]

use std::{env, fs};
use std::fs::File;


struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", &config.query);
    println!("In file {}", &config.filename);

    let content: String = fs::read_to_string(config.filename).
        expect("reading file content error");

    println!("file content is :\n{}", &content);
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config { query: args[1].clone(), filename: args[2].clone() }
    }
}
