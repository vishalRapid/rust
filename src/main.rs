// importing io module for this program, std is the library
use std::io;
use rand::Rng;

fn main() {
    println!("Let's play a game!");

    println!("Please choose a number");

    // initializing a variable mut() mutable of type String
    let mut guess = String::new(); // creating empty instance of string

    
    io::stdin().read_line(&mut guess);

    println!("You have choosen a number {guess}");


    
}
