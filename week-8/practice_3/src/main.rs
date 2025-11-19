 use std::io;

 // method to print the get value
 fn value(n:Option<&char>)
{
    println!("Element of vector is {:?}",n);
}

fn main() {
    let v = vec!['r','u','s','t','a','c','i','a','n'];

    let mut input1 = String::new();
    println!("\nEnter an index value (0-8)");
    io::stdin().read_line(&mut input1).expect("failed to read input");

    //index is the non negative value which is smaller than the size of the vector
    let index:usize = input1.trim().parse().expect("invalid input");

    //getting value at iven index value
    let ch: Option<&char> = v.get(index);
    value(ch);
}