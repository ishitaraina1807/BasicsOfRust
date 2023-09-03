use std::cmp::Ordering;
use std::env;

fn main() {
    println!("Guess the number!");

    let secret_number = 18; 

    loop {
        println!("Please input your guess (or 'exit' to quit):");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess == "exit" {
            println!("Goodbye!");
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations, you win!");
                break;
            }
        }
    }
}
