use std::io;

fn main() {
    let mut inputa = String::new();
    let mut inputb = String::new();
    let mut inputc = String::new();

    // Input coefficients
    println!("Enter the value of a:");
    io::stdin().read_line(&mut inputa).expect("Failed to read line");
    let inputa: f32 = inputa.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of b:");
    io::stdin().read_line(&mut inputb).expect("not a valid string");
    let inputb: f32 = inputb.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of c:");
    io::stdin().read_line(&mut inputc).expect("not a valid string");
    let inputc: f32 = inputc.trim().parse().expect("Please enter a valid number");

    // find discriminant
    let discriminant = (inputb * inputb) - (4.0 * inputa * inputc);

    // Determine root nature
    if discriminant > 0.0 {
        let root1 = (-inputb + discriminant.sqrt()) / (2.0 * inputa);
        let root2 = (-inputb - discriminant.sqrt()) / (2.0 * inputa);
        println!("There are two distinct roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -inputb / (2.0 * inputa);
        println!("There is exactly one real root: {}", root);
    } else {
        println!("There are no real roots.");
    }
}