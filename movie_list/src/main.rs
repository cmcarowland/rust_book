use std::io;
use std::io::Write;

struct Movie {
    title: String,
    year: i16,
    stars: i8,
}

impl Movie {
    fn new(title: String, year: i16, stars: i8) -> Movie {
        Movie {title, year, stars}
    }
    
    fn equals(&self, other: &Movie) -> bool {
        self.title == other.title && self.year == other.year
    }
}

enum Selection {
    V,
    D(Movie),
    A(Movie),
    M(Movie),
    X,
    Unknown,
}

fn main() {
    let mut movies: Vec<Movie> =  Vec::new();
    movies.push(Movie::new("Wizard Of Oz".to_string(), 1942, 5));
    println!("The Movie List Program\n");
    loop {
        print_menu();
        let sel: Selection = process_user_key(get_user_key().to_lowercase().trim());
        match sel {
            Selection::X => { break; }
            Selection::V => { 
                print_movies(&movies);
            }
            _ => {println!( "Todo"); }
        }
    }
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

fn process_user_key(key: &str) -> Selection {
    match key.chars().nth(0).unwrap() {
        'v' => {
            println!("V Pressed");
            return Selection::V;
        }
        'd' => {
            println!("D Pressed");
            return Selection::D(Movie::new("Wizard Of Oz".to_string(), 1942, 5));
        }
        'a' => {
            println!("A Pressed");
            return Selection::A(Movie::new("Wizard Of Oz".to_string(), 1942, 5));
        }
        'm' => {
            println!("M Pressed");
            return Selection::M(Movie::new("Wizard Of Oz".to_string(), 1942, 5));
        }
        'x' => {
            println!("Bye!");
            return Selection::X;
        }
        _ => {
            println!("Invalid Response");
            return Selection::Unknown;
        }
    }
}

fn print_movies(movies: &Vec<Movie>) {
    println!("\n{:<4}{:20}{:6}{:5}", "", "TITLE", "YEAR", "STARS");

    for (i, movie) in movies.iter().enumerate() {
        println!("\n{:<4}{:20}{:<6}{:<5}", i + 1, movie.title, movie.year, movie.stars);
    }

    println!();
}