use std::io;
use std::io::Write;

fn main() {
    // Open file in append mode
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("PAU_SMIS.txt")
        .expect("Failed to open file");

    loop {
        let mut name = String::new();
        let mut mat = String::new();
        let mut dept = String::new();
        let mut lvl = String::new();

        println!("Input name:");
        io::stdin().read_line(&mut name).expect("Failed to read name");

        println!("Input matric number:");
        io::stdin().read_line(&mut mat).expect("Failed to read matric");

        println!("Input department:");
        io::stdin().read_line(&mut dept).expect("Failed to read dept");

        println!("Input level:");
        io::stdin().read_line(&mut lvl).expect("Failed to read level");

        // Trim whitespace
        let name = name.trim().to_string();
        let mat = mat.trim().to_string();
        let dept = dept.trim().to_string();
        let lvl = lvl.trim().to_string();

        // Store data in a vector instead of format!()
        let student_data: Vec<String> = vec![name,mat,dept,lvl,];

        // Write each item to file on its own line
        file.write_all(b"--- Student Record ---\n").unwrap();
        file.write_all(format!("Name: {}\n", student_data[0]).as_bytes()).unwrap();
        file.write_all(format!("Matric: {}\n", student_data[1]).as_bytes()).unwrap();
        file.write_all(format!("Department: {}\n", student_data[2]).as_bytes()).unwrap();
        file.write_all(format!("Level: {}\n", student_data[3]).as_bytes()).unwrap();
        file.write_all(b"-----------------------\n\n").unwrap();

        println!("Record saved!\n");

        // Ask user if they want to continue
        let mut again = String::new();
        println!("Add another student? (yes/no):");
        io::stdin().read_line(&mut again).expect("Failed to read input");

        if again.trim().to_lowercase() != "yes" {
            println!("Exiting program. All records saved.");
            break;
        }
    }
}
