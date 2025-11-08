use std::io;

fn main() {
    println!("-------------------------------------------");
    println!("Welcome to daniel's restaurant!!!");
    println!("Our Menu:");
    println!("P = Pounded Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");
    println!("-------------------------------------------");


    let mut food = String::new();
    let mut quantity = String::new();

    println!("input food choice (P, F, A, E, W):");
    io::stdin().read_line(&mut food).expect("not a valid string");
    
    println!("enter desired quantity:");
    io::stdin().read_line(&mut quantity).expect("Failed to read line");
    let quantity: u32 = quantity.trim().parse().expect("not an unsigned integer");

    let price = match food.trim().to_uppercase().as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food choice.");
            return;
        }
    };

    let total = price * quantity;
    let mut final_total = total;

    if total > 10_000 {
        final_total = (total as f32 * 0.95) as u32; // Apply 5% discount
    }

    println!("your total cost is: N{}", final_total);
}