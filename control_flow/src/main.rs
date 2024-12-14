fn main() {

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

    let mut i = 0;
    loop {
        println!("Loop... i: {}", i);
        i = i+1;

        if i == 10 {
            break i = 100;
        }
    }

    println!("index value: {}", i);

    // Another way to get result from loop
    let mut i = 0;
    let result = loop {
        println!("Another Loop... i: {}", i);

        i = i+1;

        if i == 10 {
            break "done" // expression or statement both works
        }
    };

    println!("Another Loop.... Result: {}", result);

    // Labeled Loop
    let mut i =0;
    let result = 'labeled_loop: loop {
        println!("labeled_loop: i: {}", i);

        let mut j = 3;
        loop {
            j = j-1;

            println!("Running inner({}) loop...", j);

            if j == 0 {
                println!("Breaking inner({}) loop...", j);
                break
            }

            if i == 2 {
                println!("=> Breaking labeled_loop...");
                break 'labeled_loop "=> Main loop done"
            }
        }

        i = i+1;
    };

    println!("{}", result);

    // While loop
    let mut i =3;
    while i > 0 {
        i = i-1;
        println!("While loop: {}", i);
    }

    // for loop
    for num in (0..3).rev() {
        println!("For loop=> i: {}", num);
    }
    
    // Loop through collection
    // For-loop
    let collection = [10, 20, 30, 40, 50];
    for a in collection {
        println!("For loop with collection=> a: {}", a);
    }

    // Loop through collection
    // while-loop
    let mut i = 0;
    while i < collection.len() {
        println!("While loop with collection=> a: {}", collection[i]);
        i = i+1;
    }

    // iterator [Breif only]
    let iterator = 1..10; // from 1 to 9
    let iterator = 1..=10; // from 0 to 10
    let iterator = (0..=100).step_by(10); // 0, 10, 20.... 100
    let iterator = (1..=10).rev(); // from 10 to 1
    let iterator = (1..10).filter(|x| x%2 != 0);

    // |x| is a closure in RUST just like lambda functions in other languages
    // Basic Syntax: 
    // |parameter| body

    // so other way to apply filter()
    fn is_odd(x: i32) -> bool {
        x % 2 != 0
    }
    let iterator = (1..10).filter(|x| is_odd(*x)); // this is not cool looking right?
    
    for num in iterator {
        println!("For loop with iterator=> i: {}", num);
    }

}
