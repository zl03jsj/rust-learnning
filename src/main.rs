#![allow(unused_imports)]

use std::{env, fs, process, io};


struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", &config.query);
    println!("In file {}", &config.filename);

    let content: String = fs::read_to_string(config.filename).
        expect("reading file content error");

    println!("file content is :\n{}", &content);
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config { query: args[1].clone(), filename: args[2].clone() })
    }
}
