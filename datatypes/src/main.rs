use std::any::type_name;

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
        Err(_e) => println!("Nah! Are you sure [sample] is positive INT or INT at all?"),
    }
}
