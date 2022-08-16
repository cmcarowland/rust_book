use std::io;
use std::io::Write;

struct Movie {
    
}

fn main() {
    let mut movies: Vec<Movie>;
    println!("The Movie List Program\n");
    print_menu();
    process_user_key(get_user_key().to_lowercase().trim());
}

fn print_menu() {
    println!("COMMANDS");
    println!("v - View Movie List");
    println!("a - Add A Movie");
    println!("d - Delete A Movie");
    println!("m - Modify A Movie");
    println!("x - Exit\n");
    print!("Command: ");
    io::stdout().flush().unwrap();
}

fn get_user_key() -> String {
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    buf
}

fn process_user_key(key: &str) {
    match key.chars().nth(0).unwrap() {
        'v' => {
            println!("V Pressed");
        }
        'd' => {
            println!("D Pressed");
        }
        'a' => {
            println!("A Pressed");
        }
        'm' => {
            println!("M Pressed");
        }
        'x' => {
            println!("X Pressed");
        }
        _ => {
            println!("Invalid Response");
        }
    }
}