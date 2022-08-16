use std::io;
use std::io::Write;
use std::fs;

struct Movie {
    title: String,
    year: u16,
    stars: u16,
}

impl Movie {
    fn new(title: String, year: u16, stars: u16) -> Movie {
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
    M,
    X,
    Unknown,
}

fn main() {
    let mut movies: Vec<Movie> =  Vec::new();
    load_movies_from_file(&mut movies);

    println!("The Movie List Program\n");
    loop {
        print_menu();
        let sel: Selection = process_user_key(get_user_key().to_lowercase().trim());
        match sel {
            Selection::X => { break; }
            Selection::V => { 
                print_movies(&movies);
            }
            Selection::A(movie) => {
                add_movie(&mut movies, movie);
            }
            Selection::M => {
                print_movies(&movies);
                let index = select_movie((movies.len() as i32)+ 1);
                modify_movie(&mut movies[index as usize]);
            }
            
            _ => { println!(); }
        }
    }
}

fn load_movies_from_file(movies: &mut Vec<Movie>) {
    let movies_text = fs::read_to_string("movies.txt").unwrap();
    for line in movies_text.lines() {
        let split_line = line.split('\t').collect::<Vec<&str>>();
        movies.push(Movie::new(split_line[0].to_string(), 1942, 5));
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
            return Selection::V;
        }
        'd' => {
            return Selection::D(Movie::new("Wizard Of Oz".to_string(), 1942, 5));
        }
        'a' => {
            return Selection::A(Movie::new(get_string_from_user("Enter Title >> "), get_number_from_user("Enter Year >> "), get_number_from_user("Enter Stars >> ")));
        }
        'm' => {
            return Selection::M;
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
        println!("{:<4}{:20}{:<6}{:<5}", i + 1, movie.title, movie.year, movie.stars);
    }

    println!();
}

fn get_string_from_user(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    user_input.trim().to_string()
}

fn get_number_from_user(message: &str) -> u16 {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    match user_input.trim().parse::<u16>() {
        Ok(num) => return num,
        Err(_) => {
            println!("Invalid Number!!\nPlease Try Again...\n\n");
            return get_number_from_user(message);
        }
    }
}

fn add_movie(movies: &mut Vec<Movie>, movie: Movie) {
    movies.push(movie);
}

fn select_movie(length: i32) -> u16 {
    get_number_from_user(&format!("Select Movie (1-{})>> ", length)[..]) - 1
}

fn modify_movie(movie: &mut Movie) {
    let stars = get_number_from_user(&format!("Enter New Stars ({}) >> ", movie.stars)[..]);
    movie.stars = stars;
}