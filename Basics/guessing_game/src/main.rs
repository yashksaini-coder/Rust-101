use rand::Rng;
use std::io;

fn main() {
    println!("🎯 Welcome to the Guessing Game!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("\nEnter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Convert input to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("Congratulations! You guessed it!");
            break;
        }
    }
}
