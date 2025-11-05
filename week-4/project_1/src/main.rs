use std::io;

fn main() {

    let mut exp = String::new();
    let mut age = String::new();

    println!("input your experience (in years)");
    io::stdin().read_line(&mut exp).expect("not a valid string");
    let exp: u8 = exp.trim().parse().expect("input is not an unsigned integer");

    println!("input your age");
    io::stdin().read_line(&mut age).expect("not a valid string");
    let age: u8 = age.trim().parse().expect("input is not an unsigned integer");


    if exp > 5 && age >= 40 {
        println!("Your annual incentive is N1_560_000");
    }if exp > 5 && age >30 && age <40 {
        println!("Your annual incentive is N1_480_000");
    }if exp >5 && age <28 {
        println!("Your annual incentive is N1_300_000");
    }if exp <5 {
        println!("Your annual incentive is N100_000");
    }
}