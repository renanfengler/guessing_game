use std::io;
use rand::Rng;

fn main() {
    let answer: i32 = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number");
    println!("Plase input your guess");
    println!("number {answer}");

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");
}
