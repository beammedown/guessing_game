//use core::num::dec2flt::parse;
//use std::fmt::format;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_input() -> u32 {
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        return guess;
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    let guess = get_input();
        

    let guess2 = get_input();
    let difference = guess/secret_number;
    println!("{}", difference);
    let difference2 = guess2/secret_number;
    println!("difference 2: {}, difference2 Pointer: {}", difference2, &difference2);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
        }
}
println!("The secret number was: {}", secret_number);
}
