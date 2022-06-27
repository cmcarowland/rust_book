use std::io;
use std::io::Write;

const PI : f32 = 3.14159265359;

struct Rectangle {
    width: f32,
    height: f32,
    area: f32
}

trait Shape {
    fn new ( width: f32, height: f32) -> Self;

    fn calculate(&mut self);
}

impl Shape for Rectangle {
    fn new (w: f32, h: f32) -> Rectangle {
        return Rectangle{width: w, height: h, area: 0.0};
    }

    fn calculate(&mut self) {
        self.area = self.width * self.height;
    }
}

fn get_user_radius() -> String {
    let mut radius = String::new();

    print!("Enter Radius >> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut radius)
        .expect("Unable to get string");

    radius
}

fn check_continue() -> bool {
    let mut radius = String::new();

    print!("Continue? (y/n) >> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut radius)
        .expect("Unable to get string");

    radius.chars().nth(0).unwrap() == 'y'
}

fn convert_radius_to_float(radius : &String) -> f32 {
    let radius: f32 = match radius.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Not a number");
            -1.0
        },
    };

    radius
}

fn calculate_diameter(radius: f32) -> f32 {
    radius * 2.0
}

fn calculate_circumference(diameter: f32) -> f32 {
    diameter * PI
}

fn calculate_area(radius: f32) -> f32 {
    PI * f32::powf(radius, 2.0)
}

fn main() {
    let mut diameter;
    let mut circumference;
    let mut area;
    let mut is_running : bool = true;

    while is_running {
        let radius = get_user_radius();

        let radius = convert_radius_to_float(&radius);

        diameter = calculate_diameter(radius);
        circumference = calculate_circumference(diameter);
        area = calculate_area(radius);

        println!("Radius : {}", radius);
        println!("Diameter : {}", diameter);
        println!("Circumference : {}", circumference);
        println!("Area : {}", area);

        let mut r = Rectangle::new(3.0, 4.0);
        r.calculate();
        println!("Rect Area : {}", r.area); 

        is_running = check_continue();
    }
}
