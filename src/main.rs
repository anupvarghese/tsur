extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the letter!");
    println!("Pls enter your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read the letter");
    println!("Your guess: {}", guess);
    let random = rand::thread_rng().gen_range(1, 10);
    println!("Computer guessed: {}", random);
}
