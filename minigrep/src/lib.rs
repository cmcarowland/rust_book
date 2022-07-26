use std::fs;
use std::error::Error;
use colored::*;
use std::env;
//702-410-4651 ctn1 Grines

pub struct Config {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool,
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
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{ query, filename, case_sensitive })
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new(); 
    let lower_query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&lower_query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents : String = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }
    else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
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
            Ok(_c) => {
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
            Ok(_c) => {
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
            Ok(_c) => {
                assert!(false);
            },
            Err(e) => {
                println!("Error : {}", e);
                assert!(true);
            }
        }
    }

    #[test]
    fn search_test_case_sensitive() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search("duct", contents))
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}