#![allow(unused_imports)]

use std::{env, fs, process, io, error::Error};

use hello_cargo::mini_grep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", &config.query);
    println!("In file {}", &config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

