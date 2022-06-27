use std::io;
use std::io::Write;

fn main() {
    let mut customer_type = String::new();
    let mut sub_total = String::new();
    let discount_percent: f64;
    let discount_amount: f64;
    let invoice_total: f64;
    
    println!("Invoice Calculator 2.0");

    print!("Enter Customer Type (r/w/c) : ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut customer_type)
        .expect("Cannot read line");

    print!("Enter SubTotal : ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut sub_total)
        .expect("Subtotal Failed to read line");

    let sub_total: f64 = match sub_total.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error parsing subtotal");
            return;
        }
    };

    match customer_type.trim() {
        "r" => {
            if sub_total < 100.0 {
                discount_percent = 0.0;
            }
            else if sub_total >= 100.0 && sub_total < 250.0 {
                discount_percent = 0.1;
            }
            else if sub_total >= 250.0 && sub_total < 500.0 {
                discount_percent = 0.2;
            }
            else {
                discount_percent = 0.3;
            }
        },
        "c" => {
            discount_percent = 0.25;
        },
        "w" => {
            if sub_total < 500.0 {
                discount_percent = 0.4;
            }
            else {
                discount_percent = 0.5;
            }
        },
        _ => {
            println!("Unknown Customer Type");
            discount_percent = 0.0;
        }
    } 

    println!("Discount Percent : {:.2}", discount_percent);

    discount_amount = sub_total * discount_percent;
    invoice_total = sub_total - discount_amount;

    println!("Discount Amount : {:.2}", discount_amount);
    println!("Invoice Total : {:.2}", invoice_total);
}
