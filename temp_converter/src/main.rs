use std::io;
use std::io::Write;

#[derive(Debug)]
enum TempType {
    F,
    C,
}

struct Temperature {
    fahrenheit: f32,
    celsius: f32,
}

impl Temperature {
    fn new() -> Temperature {
        Temperature{fahrenheit: 32.0, celsius: 0.0}
    }

    pub fn set_value(&mut self, temp_type: TempType, value: f32) {
        match temp_type {
            TempType::F => self.set_f(value),
            TempType::C => self.set_c(value),
        }
    }

    fn set_f(&mut self, val : f32) {
        self.fahrenheit = val;
        self.celsius = (val - 32.0) * 5.0 / 9.0;
        self.round();
    }

    fn set_c(&mut self, val : f32) {
        self.celsius = val;
        self.fahrenheit = val * 9.0 / 5.0 + 32.0;
        self.round();
    }

    fn round(&mut self) {
        let c: i32 = (self.celsius * 1000.0) as i32;
        self.celsius = c as f32 / 1000.0;

        let f: i32 = (self.fahrenheit * 1000.0) as i32;
        self.fahrenheit = f as f32 / 1000.0;
    }
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Celcius: {}\nFahrenheit: {}", self.celsius, self.fahrenheit)
    }
}

fn main() {
    let mut t = Temperature::new();
    
    print!("Enter Temperature Type (F or C) >> ");
    io::stdout().flush().unwrap();

    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let temp_type = match buf.trim().chars().nth(0).unwrap() {
        'F' => TempType::F,
        'f' => TempType::F,
        'C' => TempType::C,
        'c' => TempType::C,
        _ => {
            println!("Invalid Option");
            return;
        }
    };

    buf.clear();
    print!("Enter Temperature >> ");
    io::stdout().flush().unwrap();

    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let user_temp: f32 = buf.trim().parse().unwrap();

    t.set_value(temp_type, user_temp);
    println!("\n\n{}\n", t);
}
