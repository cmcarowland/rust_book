use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    println!("Searching for {}", query);
    println!("In File {}", filename);
    println!("With Text :\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}