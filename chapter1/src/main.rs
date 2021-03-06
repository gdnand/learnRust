extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your number: ");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number");
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Less!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You win!"),
    }
    println!("The Secret number is {}", secret_number);
}
