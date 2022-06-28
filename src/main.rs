#![allow(unused_imports)]

use std::{env, fs, process, io, error::Error};

pub mod mini_grep;

use mini_grep::{search, Config, run};

fn main() {
    let some_number: Option<i32> = None;
    // map_or, 如果some_number不是none, 则将原始值带入后面的函数中(n+1), 否则返回默认值:2
    let x = some_number.map_or(2, |n| n + 1);
    println!("number = {}", x);

    // map_or, 如果some_number不是none, 则将原始值带入后面的函数中(n*2), 否则执行第一个函数, 这里是直接返回32.
    let x = some_number.map_or_else(|| 32, |n| n*2);
    println!("number = {}", x);

    let some_number = Some(9);

    // map, 如果some_number为Some, 则执行函数, 并继续往下走, 否则直接返回None.
    let another_number = some_number
        .map(|n| n - 1) // => Some(8)
        .map(|n| n * n) // => Some(64)
        .and_then(|n| divide(n, 4)); // => Some(16)

    println!("{}", to_message(another_number));

    let final_number = another_number
        .and_then(|n| divide(n, 0));

    println!("{}", to_message(final_number));
}

fn divide(number: i32, divisor: i32) -> Option<i32> {
    if divisor != 0  { Some(number/divisor) } else { None }
}

fn to_message(number: Option<i32>) -> String {
    number
        .map(|n| format!("{} is definitely a number!", n)) // => Some("...")
        .unwrap_or("None!".to_string()) // => "..."
}

// fn main() {
//     let blocks :&[&[u8]] =&[&[1,2,3], &[3, 4, 5]];
//
//     let args: Vec<String> = env::args().collect();
//
//     let config: Config = Config::new(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });
//
//     println!("Searching for {}", &config.query);
//     println!("In file {}", &config.filename);
//
//     if let Err(e) = run(config) {
//         println!("Application error: {}", e);
//         process::exit(1);
//     }
// }
//
