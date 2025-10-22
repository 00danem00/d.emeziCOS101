use std::io; //import crate

fn main() {

    // generate a mutable string variable

    let mut experience = String::new();
    let mut age = String::new();

    println!("input your experience (in years):");
    io::stdin().read_line(&mut experience).expect("not a valid string"); //generate a variable stdin with an instance of io::stdin() 
    let experience: u8 = experience.trim().parse().expect("input is not an unsigned integer");

    println!("input your age:");
    io::stdin().read_line(&mut age).expect("not a valid string");
    let age: u8 = age.trim().parse().expect("input is not an unsigned integer");

    // Determining the annual incentive
   fn calculate_incentive(expeience: u8, age: u8) {
   	if experience >= 5 && age >= 40 {
   		1_560_000
   	}else if experience >= 5 && age >=30 && age <40{
   		1_480_000
   	}else if experience >= 5 && age <28 {
   		1_300_000
   	}else if experience < 5 && age <28{
   		100_000
   	}
   }
   println!("your annual incentive is: N{}", calculate_incentive );

}