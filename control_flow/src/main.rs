fn main() {

    let mut i = 0;
    loop {
        println!("Loop... i: {}", i);
        i = i+1;

        if i == 10 {
            break i = 100;
        }
    }

    println!("index value: {}", i);

    let x = 5;

    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is less than or equal to 5");
    }

    // ---- error
    // if x {
    //     println!("x is bool");
    // }

    if x != 1 {
        println!("x is not equal to 1");
    }

    if x % 2 == 0 {
        println!("x is divisible by 2");
    }

    if x % 5 == 0 {
        println!("x is divisible by 5");
    }

    let x = if x > 5 { 6 } else { 4 };

    println!("now x value is changed to: {}", x);
}
