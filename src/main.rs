use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number");
    println!("Plase input your guess");
    println!("number {secret_number}");

    loop {
        let mut guess  = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type only numbers!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break; // or return;
            },
            Ordering::Greater => println!("To big!"),
        }
    }
}
