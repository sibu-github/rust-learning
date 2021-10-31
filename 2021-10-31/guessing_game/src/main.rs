use rand::prelude::*;
use std::io::{self, BufRead};

fn main() {
    let mut rng = thread_rng();
    let random_number: i32 = rng.gen_range(0..100);
    println!("please enter your guess.");
    let mut input_text = String::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    let mut trimmed = input_text.trim();
    let mut guess = trimmed.parse::<i32>().unwrap();
    loop {
        input_text.clear();
        if random_number == guess {
            println!("You have guessed correct number");
            break;
        } else {
            if random_number > guess {
                println!("too low!, {}", guess);
            } else {
                println!("too high!, {}", guess);
            }

            println!("please enter your guess again.");
            handle
                .read_line(&mut input_text)
                .expect("failed to read from stdin");
            trimmed = input_text.trim();
            guess = trimmed.parse::<i32>().unwrap();
        }
    }

    println!(
        "random number is {} and the guess is {}",
        random_number, guess
    );
}
