use std::error::Error;
use std::fs;

// Grouping Configuration Values
pub struct Config {
    pub query: String,
    pub file_path: String,
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

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
