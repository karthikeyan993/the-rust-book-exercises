use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

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

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if (secret_number - guess) < 10 {
                    println!("Small");
                } else {
                    println!("Too small!");
                }
            }
            Ordering::Greater => {
                if (guess - secret_number) > 10 {
                    println!("Too big!")
                } else {
                    println!("big")
                }
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
