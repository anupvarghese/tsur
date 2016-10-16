extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the letter!");
    println!("Pls enter your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read the number");
    println!("Your guess: {}", guess);
    let guess: u32 = guess.trim().parse().expect("Pls enter a number");
    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("Computer guessed: {}", secret_number);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Smaller bet, lost"),
        Ordering::Greater => println!("Larger bet, lost"),
        Ordering::Equal => println!("You win"),
    }
}
