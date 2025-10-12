fn main() {
    let salesamounts: [f64; 5] = [
        450_000.00,  
        1_500_000.00, 
        750_000.00,   
        2_850_000.00, 
        250_000.00];

let totalsales: f64 = salesamounts.iter().sum();
println!("Total Sales is {}", totalsales);
let averagesales = totalsales / salesamounts.len() as f64;
println!("Average Sales is {}", averagesales); 
}