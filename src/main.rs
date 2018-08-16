extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let random_number = rand::thread_rng().gen_range(1,10); 

    println!("{}", random_number);
    println!("Input your guess!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You win!"),
    }
}
