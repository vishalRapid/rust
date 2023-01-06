use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Lets plan!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please choose a number! \n ->");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // creating a new variable with u32 type that means signed integer
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You have choosen {guess}, but secret number is {secret_number}");

    // comparison
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Number is small!"),
        Ordering::Greater => println!("Number is bigger"),
        Ordering::Equal => println!("You have guessed the right number !"),
    }
}
