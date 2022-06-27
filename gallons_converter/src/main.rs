use std::io;
use std::io::Write;

const GALLONS_TO_LITERS: f32 = 3.7854;
const GALLONS_TO_QUARTS: f32 = 4.0;
const GALLONS_TO_OUNCES: f32 = 128.0;

fn main() {
    println!("Welcome to the Gallon Converter");
    
    print!("Enter Gallons >> ");
    io::stdout().flush().unwrap();

    let mut gallons = String::new();
    io::stdin().read_line(&mut gallons)
        .expect("Failed To read Line");
    
    let gallons: f32 = match gallons.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Number Entered");
            return;
        },
    };

    println!("Gallons : {}", gallons);
    
    let liters: f32 = gallons * GALLONS_TO_LITERS;
    let quarts: f32 = gallons * GALLONS_TO_QUARTS;
    let ounces: f32 = gallons * GALLONS_TO_OUNCES;

    println!("Liters : {:.2}", liters);
    println!("Quarts : {:.2}", quarts);
    println!("Ounces : {:.2}", ounces);

}
