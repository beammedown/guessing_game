
fn main() {
    println!("**CLOSEST NUMBER WINS**");
    println!("Guess the number!");

}

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