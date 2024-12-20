use std::any::type_name;
use std::io;

fn get_typename<T>(_: &T) -> &str {
    return type_name::<T>();
}

fn main() {
    println!("Let's talk about DataTypes in RUST!");

    let sample = "-42";
    println!("sample type:: {}", get_typename(&sample));

    let makeit_number: i32 = sample.parse().expect("Not a number!");
    println!("now sample type:: {}", get_typename(&makeit_number));

    let sample_parse: Result<u32, _> = sample.parse();
    match sample_parse {
        Ok(_) => println!("Hurray! [sample] is parsed as unsigned INT"),
        Err(_e) => println!("ERROR:: Are you sure [sample] is positive INT or INT at all?"),
    }

    // Cool feature of RUST :D
    let suffix_literal = -57i8;
    println!("Suffix Literal => {}", suffix_literal.to_string());
    
    let suffix_literal = 57u8;
    println!("Suffix Literal => {}", suffix_literal.to_string());

    let suffix_literal = 64usize;
    println!("Suffix Literal => {}", suffix_literal.to_string());

    // Another Cool feature of RUST=> Visual Separator
    let easier_to_read = 1_00_000;
    println!("Easier To Read => {}", easier_to_read.to_string());

    let my_int = 98_222;
    let my_int = 0xff;
    let my_int = 0o77;

    // Let's create overflow-int
    // let overflow_int = 256u8;
    // Now, technically its incorrect and it will throw error... coz u8 can only hold values between 0-255


    // Float Type
    let fx = 65.12;
    let fy = 65.12f32;

    // Calculation
    println!("Add: fx+fy = {}", fx+fy);
    println!("Sub: my_int-easier_to_read = {}", (my_int - easier_to_read));
    println!("Product: my_int*2 = {}", my_int*2);
    println!("Divide: my_int/4 = {}", my_int/4);
    println!("Truncated: -5/3 = {}", -5/3);
    println!("Remainder: 43%2 = {}", 43%2);

    // Boolean
    let my_bool = false;
    println!("my_bool: {}", my_bool);

    // Chars
    let my_str = "Hello World!";
    let my_char: char = 'Z';
    let my_emoji = "😻";

    println!("My String: {}", my_str);
    println!("My Char: {}", my_char);
    println!("My Emoji: {}", my_emoji);

    // Tuple
    let tup = (my_int, my_bool, my_str);
    println!("My Tuple: {}", tup.2);

    let (x, y, z) = tup;
    println!("My Tuple: {}", y);

    // Array
    let array1 = [1,2,3,4,5];

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    println!("Please entry index of month::");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
                        .trim()
                        .parse()
                        .expect("Please enter valid month index");

    let month = months[index];

    println!("Month:: {}", month);

    let array2 = [100; 3];
    
    for n in array1 {
        println!("array1 element: {}", n);
    }

    for n in array2 {
        println!("array2 element: {}", n);
    }
    
}