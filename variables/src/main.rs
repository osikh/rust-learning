use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("TypeName-> {}", type_name::<T>());
}

fn main() {
    // this_will_not_work
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // this_will_work
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const {THREE_HOURS_IN_SECONDS}");


    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("var:x in limited scope has value={x}")
    }

    println!("shadowed var:x has value={x}");


    // benefit of shadowing: you can change type of variable
    // because technically you are defining the variable again using the same name... cool feature isn't it
    let spaces = "   ";
    print_type_of(&spaces);

    let spaces = spaces.len();
    print_type_of(&spaces);

    // You can't change type with muttable variables
    let mut spaces = " ";
    print_type_of(&spaces);

    let spaces_length = spaces.len().to_string();
    print_type_of(&spaces_length);

}
