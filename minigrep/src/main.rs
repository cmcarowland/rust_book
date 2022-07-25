use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments: {}", err);
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    run(config);
}

struct Config {
    query : String,
    filename : String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3  {
            return Err("Not enough arguments!!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{ query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With Text :\n{}", contents); 

    Ok(())
}