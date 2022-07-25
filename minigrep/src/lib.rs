use std::fs;
use std::error::Error;

pub struct Config {
    pub query : String,
    pub filename : String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3  {
            return Err("Not enough arguments!!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{ query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With Text :\n{}", contents); 

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let args : [String; 3] = ["a".to_string(), "b".to_string(), "poem.txt".to_string()];
        let config = Config::new(&args).unwrap();
        
        if let Err(e) = run(config) {
            assert!(false, "Failed to run properly: {}", e);
        }
        
    }
}