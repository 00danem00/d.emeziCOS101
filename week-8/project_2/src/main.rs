use std::io;

fn main() {
    let mut candidates = Vec::new();

    loop {
        let mut name = String::new();
        println!("Enter candidate's name (or 'quit' to finish):");
        io::stdin().read_line(&mut name).expect("Error reading input");
        let name = name.trim();

        if name == "quit" {
            break;
        }

        let mut experience_str = String::new();
        println!("Enter years of experience:");
        io::stdin().read_line(&mut experience_str).expect("Error reading input");
        let experience: u32 = experience_str.trim().parse().expect("Please enter a valid number");

        candidates.push((name.to_string(), experience));
    }

    println!("\nCandidates with more than 5 years of experience:");
    for (name, experience) in candidates {
        if experience > 5 {
            println!("{}, {} years", name, experience);
        }
    }
}