use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let countries = HashMap::from([
        ("CA", "Canada"),
        ("MX", "Mexico"),
        ("US", "United States"),
    ]);

    display_header();
    display_country_codes(&countries);

    loop {
        let country_code = get_user_country_code();

        if countries.contains_key(&country_code as &str) {
            println!("\nYou selected {}!\n", countries[&country_code as &str]);
        } else {
            println!("\nCountry code {} not found\n", country_code);
        }

        if !check_continue() {
            break;
        }
    }
}

fn display_header() {
    println!("The Country Codes Program\n------------------------\n");
}

fn display_country_codes(countries: &HashMap<&str, &str>) {
    print!("Country Codes: ");
    for (k, _) in countries {
        print!("{} ", k);
    }
    println!();
}

fn check_continue() -> bool {
    let mut buf = String::new();
    print!("Continue (y/n) :");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    match buf.trim() {
        "n" => {
            false
        }
        _ => {
            true
        }
    }
}

fn get_user_country_code() -> String {
    let mut buf = String::new();
    print!("Enter Country Code : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    buf.trim().to_string().to_uppercase()
}