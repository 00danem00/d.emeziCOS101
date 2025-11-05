fn main() {
    let name1 = "Ayoide Adesokan";
    println!("My name is {}", name1);

    // Find and replace
    let name2 = name1.replace("Ayoide", "Adebare");
    println!("You can also call me {},", name2);

    // Faculty
    let faculty = "Faculty of Science and Technology";
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}", school);
}