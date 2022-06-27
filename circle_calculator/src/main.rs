use std::io;
use std::io::Write;

struct Rectangle {
    width: f32,
    height: f32,
    area: f32
}

trait Shape {
    fn new ( width: f32, height: f32) -> Self;

    fn Calculate(&mut self);
}

impl Shape for Rectangle {
    fn new (w: f32, h: f32) -> Rectangle {
        return Rectangle{width: w, height: h, area: 0.0};
    }

    fn Calculate(&mut self) {
        self.area = self.width * self.height;
    }
}

fn main() {
    let mut radius = String::new();
    let pi = 3.14;
    let diameter;
    let circumference;
    let area;

    print!("Enter Radius >> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut radius)
        .expect("Unable to get string");

    let radius: f32 = match radius.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Not a number");
            return;
        },
    };

    diameter = radius * 2.0;
    circumference = diameter * pi;
    area = pi * f32::powf(radius, 2.0);

    println!("Radius : {}", radius);
    println!("Diameter : {}", diameter);
    println!("Circumference : {}", circumference);
    println!("Area : {}", area);

    let mut r = Rectangle::new(3.0, 4.0);
    r.Calculate();
    println!("Rect Area : {}", r.area); 
}
