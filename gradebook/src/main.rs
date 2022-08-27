//Make a gradebook application that can 
//[ ] Take user input for a series of grades
//[ ] Return the average if a negative number is entered.

use std::io;
use std::io::Write;

fn main() {
    let mut scores : Vec<i32> = Vec::new();
    let mut grade: String;

    println!("Enter Grades (0 or less to exit)");
    io::stdout().flush().unwrap();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        grade = String::new();
        io::stdin().read_line(&mut grade)
            .expect("Failed To Read Line.");

        let grade: i32 = match grade.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Number Entered!");
                return;
            }
        };

        if grade > 0 {
            add_score(grade, &mut scores);
        } else {
            break;
        }
    }

    println!("Scores Entered {:?}", scores);
    println!("Score {:.2}", average_scores(&scores));
}

fn add_score(score: i32, scores: &mut Vec<i32>) {
    scores.push(score);
}

fn average_scores(scores : &Vec<i32>) -> f32 {
    (scores.iter().sum::<i32>() as f32) / (scores.len() as f32)
}