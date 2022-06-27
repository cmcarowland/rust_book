use std::io;
use std::io::Write;

fn main() {
    let mut customerType = String::new();
    let mut subTotal = String::new();
    let discountPercent: f64;
    let discountAmount: f64;
    let invoiceTotal: f64;
    
    println!("Invoice Calculator 2.0");

    print!("Enter Customer Type (r/w/c) : ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut customerType)
        .expect("Cannot read line");

    print!("Enter SubTotal : ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut subTotal)
        .expect("Subtotal Failed to read line");

    let subTotal: f64 = match subTotal.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error parsing subtotal");
            return;
        }
    };

    match customerType.trim() {
        "r" => {
            if subTotal < 100.0 {
                discountPercent = 0.0;
            }
            else if subTotal >= 100.0 && subTotal < 250.0 {
                discountPercent = 0.1;
            }
            else if subTotal >= 250.0 && subTotal < 500.0 {
                discountPercent = 0.2;
            }
            else {
                discountPercent = 0.3;
            }
        },
        "c" => {
            discountPercent = 0.25;
        },
        "w" => {
            if subTotal < 500.0 {
                discountPercent = 0.4;
            }
            else {
                discountPercent = 0.5;
            }
        },
        _ => {
            println!("Unknown Customer Type");
            discountPercent = 0.0;
        }
    } 

    println!("Discount Percent : {:.2}", discountPercent);

    discountAmount = subTotal * discountPercent;
    invoiceTotal = subTotal - discountAmount;

    println!("Discount Amount : {:.2}", discountAmount);
    println!("Invoice Total : {:.2}", invoiceTotal);
}
