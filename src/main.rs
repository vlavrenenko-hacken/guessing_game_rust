use std::{io, cmp::Ordering};
// io library from std library
use rand::Rng;


fn main() {
    println!("Guess the number!");

    loop {
    println!("\nPlease input your guess");
    let answer: u32 = generate_rand();
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read the line");
    
    
    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&answer) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You won, baby!");
            break;
        }
    }
    }
}

fn generate_rand() -> u32 {
    let rand_number:u32 = rand::thread_rng().gen_range(1..=100);
    return rand_number; 
}