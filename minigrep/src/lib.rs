use std::error::Error;
use std::{env, fs};

// Grouping Configuration Values
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// Creating a Constructor for Config
impl Config {
    // fn new(args: &[String]) -> Config {
    //     // Improving the Error Message
    //     if args.len() < 3 {
    //         panic!("not enough arguments");
    //     }

    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Config { query, file_path }
    // }

    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }

    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     let ignore_case = env::var("IGNORE_CASE").is_ok();

    //     Ok(Config {
    //         query,
    //         file_path,
    //         ignore_case,
    //     })
    // }
    // Using Iterator Trait Methods Instead of Indexing
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(args) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// Extracting the Argument Parser
// fn parse_config(args: &[String])-> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//
//     Config {query, file_path}
// }

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }

// Making Code Clearer with Iterator Adaptors
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
