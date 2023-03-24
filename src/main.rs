use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

use rand::Rng;

fn main() {
    println!("—— Welcome to Guess Number Game ——");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Pls guess a number: ");
        stdout().flush().ok();

        let mut input_string = String::new();
        match stdin().read_line(&mut input_string) {
            Ok(_) => {}
            Err(err) => {
                println!("[Error] while reading input: {}", err);
                continue;
            }
        }

        let input_number: u32 = match input_string.trim().parse::<u32>() {
            Ok(num) => num,
            Err(err) => {
                println!("[Error] while parsing input: {}", err);
                continue;
            }
        };

        match input_number.cmp(&secret_number) {
            Ordering::Less => {
                println!("You guessed {input_number}, it's *Less* than the secret_number!")
            }
            Ordering::Equal => {
                println!("Congratulations! The secret number is {secret_number}");
                break;
            }
            Ordering::Greater => {
                println!("You guessed {input_number}, it's *Greater* than the secret_number!")
            }
        }
    }
}
