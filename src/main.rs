use std::io;
//use std::env;

fn main() {
    
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");   w3345233
    
    /*
    for argument in env::args() {
    println!("{argument}");
    }
    */
}