use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // first computer decide
    let real_number = rand::thread_rng().gen_range(1..3);
    
    println!("Your move (between 1-3)::");
    let mut guess = String::new();

    // now, let user guess the number
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // --- Method 1 ---
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // match guess.cmp(&real_number) {
    //     std::cmp::Ordering::Less => println!("Naah!"),
    //     std::cmp::Ordering::Equal => println!("You WIN!"),
    //     std::cmp::Ordering::Greater => println!("Naah!"),
    // }
    
    // --- Method 2 ---
    let player_number = guess.trim().parse();

    match player_number {
        Ok(n) => {
            let win = if real_number == n { "YES" } else { "NO" };
            println!("Computer: {real_number}\t\tPlayer: {n}\t\tWin?: {win}", )
        }
        Err(e) => println!("Failed to convert string to i32: {}", e)
    }

}
