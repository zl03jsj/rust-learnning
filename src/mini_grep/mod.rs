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
