fn main() {
    //initialize a mutable tuple
    let mut mountain_heights= ("Everest", 8848, "Fishtail", 6993);
    println!("\nOriginal tuple: {:?}", mountain_heights);

    //change 3rd and 4th element of the mutable tuple
    mountain_heights.2 = "Fishbraid";
    mountain_heights.3 = 69930;

    println!("Changed tuple: {:?}\n", mountain_heights);
}
