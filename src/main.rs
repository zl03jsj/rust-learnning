#![allow(unused_imports)]

use std::{env, fs};
use std::fs::File;


struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let content: String = fs::read_to_string(filename).expect("reading file content error");

    println!("file content is :\n{}", &content);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
