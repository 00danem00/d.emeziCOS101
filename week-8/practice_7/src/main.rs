fn main() {
    // initialization of tuple with datatype
    let datatype_tuple: (&str, f32, u8)=("rust", 3.14, 100);
    println!("\nTuple_1 contents: {:?}", datatype_tuple);

    //initialization of tuple without data type
    let no_datatype_tuple: (&str, u16, char, f32) = ("byee",134, 'c', 1.34);
    println!("Tuple_2 contents: {:?}", no_datatype_tuple);

    //accessing tuple element at index 0
    println!("First tuple element: {:?}", no_datatype_tuple.0);

    //accessing tuple element at index 1
    println!("Second tuple element: {:?}", no_datatype_tuple.1);

    //accessing tuple element at index 2
    println!("Third tuple element: {:?}", no_datatype_tuple.2);

    //accessing tuple element at index 3
    println!("Fourth tuple element: {:?}\n", no_datatype_tuple.3);
}