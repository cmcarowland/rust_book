use rand::Rng;
use std::io::Write;
use std::io;

struct Die {
    value: i32,
    six: String,
    five: String,
    four: String ,
    three: String,
    two: String  ,
    one: String  ,
}

impl Die {
    fn new() -> Die {
        Die {
            value: 0,
            six:   " _____ \n|o   o|\n|o   o|\n|o___o|\n".to_string(),
            five:  " _____ \n|o   o|\n|  o  |\n|o___o|\n".to_string(),
            four:  " _____ \n|o   o|\n|     |\n|o___o|\n".to_string(),
            three: " _____ \n|o    |\n|  o  |\n|____o|\n".to_string(),
            two:   " _____ \n|o    |\n|     |\n|____o|\n".to_string(),
            one:   " _____ \n|     |\n|  o  |\n|_____|\n".to_string(),
        }
    }

    fn set_value(&mut self, val: i32) {
        self.value = val;
    }

    fn get_value(&self) -> i32 {
        self.value
    }

    fn get_image(&self) -> &str {
        match self.value {
            1 => return &self.one[..],
            2 => return &self.two[..],
            3 => return &self.three[..],
            4 => return &self.four[..],
            5 => return &self.five[..],
            6 => return &self.six[..],
            _ => return "Invalid Die Number",
        }
    }

    fn roll(&mut self) {
        self.set_value(rand::thread_rng().gen_range(1, 7));
    }
}

struct Roll {
    dice: Vec<Die>,
    score: i32,
}

impl Roll {
    fn new(num_of_die: i32) -> Roll {
        let mut dice: Vec<Die> = Vec::new();
        (0..num_of_die).for_each(|_i| {
            dice.push(Die::new());
        });

        Roll{dice: dice, score: 0}
    }

    fn roll_all(&mut self) {
        for die in self.dice.iter_mut() {
            die.roll();
        }

        self.score_all();
    }

    fn print_dice(&self) {
        for die in self.dice.iter() {
            println!("{}", die.get_image());
        }
    }

    fn score_all(&mut self) {
        let mut s: i32 = 0;

        for die in self.dice.iter() {
            s += die.get_value();
        }

        self.score = s;
    }

    fn get_score(&self) -> i32 {
        self.score
    }
}

fn main() {
    println!("THE DICE ROLLER PROGRAM");
    println!("-----------------------");
    loop {
        print!("Enter Number of Dice To Roll >> ");
        io::stdout().flush().unwrap();

        let mut str_dice: String = String::new();
        io::stdin().read_line(&mut str_dice).unwrap();
        let dice_num: i32 = str_dice.trim().parse().unwrap();

        let mut roll = Roll::new(dice_num);

        roll.roll_all();
        roll.print_dice();
        
        println!("{}", roll.get_score());
        
        //Continue?
        print!("Continue (y/Y) : ");
        io::stdout().flush().unwrap();

        str_dice.clear();
        io::stdin().read_line(&mut str_dice).unwrap();
        println!("{}", str_dice);
        match str_dice.chars().nth(0).unwrap() {
            'Y' => continue,
            'y' => continue,
            _ => break,
        }
    }
}
