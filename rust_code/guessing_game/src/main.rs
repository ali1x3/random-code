use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a Number!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Large!"),
        }

    }
}
