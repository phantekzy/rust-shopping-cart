use std::io::{self, Write};

fn main() {
    let currency = '$';
    // 1. Get Item Name
    print!("What item would you buy? : ");
    io::stdout().flush().unwrap();
    let mut item = String::new();
    io::stdin()
        .read_line(&mut item)
        .expect("Failed to read the line");
    let mut item = item.trim().to_string();

    // 2. Get Price
    print!("What is the price of this item? : ");
    io::stdout().flush().unwrap();
    let mut price_input = String::new();
    io::stdin()
        .read_line(&mut price_input)
        .expect("Failed to read");
    let price: f64 = price_input.trim().parse().expect("Please type a number");

    // 3. Get Quantity
    print!("How many would you like to buy ? :");
    io::stdout().flush().unwrap();
    let mut qty_input = String::new();
    io::stdin()
        .read_line(&mut qty_input)
        .expect("Failed to read");
    let quantity: i32 = qty_input
        .trim()
        .parse()
        .expect("Please type a whole number");

    // Logic
    let total = price * (quantity as f64);

    if quantity > 1 {
        item.push('s');
    }
    println!("\n--- RECEIPT ---");
    println!("Item:     {item}");
    println!("Price:    {currency}{price:.2}"); // :.2 formats to 2 decimal places
    println!("Quantity: {quantity}");
    println!("Total:    {currency}{total:.2}");
}
