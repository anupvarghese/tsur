use std::io;

fn main() {
    println!("Guess the letter!");
    println!("Pls enter your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read the letter");
    println!("Your guess: {}", guess);
}
