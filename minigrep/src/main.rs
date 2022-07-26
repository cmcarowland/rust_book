use std::env;
use std::process;
use colored::*;

use minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguments: {}", err.red());
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);

        process::exit(1);
    }
}



