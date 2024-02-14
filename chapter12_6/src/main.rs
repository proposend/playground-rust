use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Printing Errors to Standard Error eprint!
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
