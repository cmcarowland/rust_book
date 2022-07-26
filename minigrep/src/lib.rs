use std::fs;
use std::error::Error;
use colored::*;
//702-410-4651 ctn1 Grines

pub struct Config {
    pub query : String,
    pub filename : String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3  {
            return Err("Not enough arguments!!");
        }
        else if args.len() > 3 {
            return Err("Too many Arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{ query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new(); 
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents : String = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line.bold());
    }

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

    #[test]
    fn new_test_correct_args() {
        let args : [String; 3] = ["a".to_string(), "b".to_string(), "poem.txt".to_string()];
        let config = Config::new(&args);

        match config {
            Ok(c) => {
                assert!(true);
            },
            Err(e) => {
                println!("Error : {}", e);

                assert!(false);
            }
        }
    }

    #[test]
    fn new_test_fewer_args() {
        let args : [String; 2] = ["a".to_string(), "b".to_string()];
        let config = Config::new(&args);

        match config {
            Ok(c) => {
                assert!(false);
            },
            Err(e) => {
                println!("Error : {}", e);
                assert!(true);
            }
        }
    }
   
    #[test]
    fn new_test_too_many_args() {
        let args : [String; 4] = ["a".to_string(), "b".to_string(), "poem.txt".to_string(), "c".to_string()];
        let config = Config::new(&args);

        match config {
            Ok(c) => {
                assert!(false);
            },
            Err(e) => {
                println!("Error : {}", e);
                assert!(true);
            }
        }
    }

    #[test]
    fn search_test() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search("duct", contents))
    }
}