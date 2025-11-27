use std::io::Write;

fn main() {

    let mut file = std::fs::File::create("brewery_categories.txt").expect("create failed");
    file.write_all("LAGER: 33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n".as_bytes()).expect("failed to write");
    file.write_all("STOUT: Legend, Turbo King, Williams\n".as_bytes()).expect("failed to write");
    file.write_all("NON-ALCOHOLIC: Maltina, Amstel Malta, Malta Gold, Fayrouz\n".as_bytes()).expect("failed to write");

    println!("File has been created");
}