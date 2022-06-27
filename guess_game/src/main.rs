use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let rand_num: u8 = rand::thread_rng().gen_range(1, 101);
    let mut guesses:u32 = 0;

    println!("Guess Game");
    loop{
        guesses += 1;
        print!("Enter Number >> ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rand_num){
            Ordering::Less => println!("\n^^^^^ Guess Higher ^^^^^^\n"),
            Ordering::Greater => println!("\nvvvvvv Guess Lower vvvvvv\n"),
            Ordering::Equal => break,
        }
    }

    println!("Great Job!!!  You got it in {} guesses.", guesses);
}
