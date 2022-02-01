/*
Project: guessing game
File: main.rs
Author: Chase Ruskin
Abstract:
    Simple number guessing game.

    A learning module taken from The Rust Programming
    Language book.
*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// num++  
fn increment(num: u32) -> u32 {
    return num + 1;
}

fn main() {
    //number of possible numbers to select from
    let max_num = 256;

    //track the number of guesses
    let mut attempts = 0;

    println!("Guess the number! (1-{})", max_num);

    //generate a random number 1-100
    let secret_number = rand::thread_rng().gen_range(1..max_num + 1);

    //string formatting (comment this line to hide the answer!)
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        //create a new mutable string
        let mut guess = String::new();

        //accept user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //ignore a non-number guess and ask for another guess
        //shadow the 'guess' identifier to keep name as new datatype
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //catch-all errors with '_'
        };

        println!("You guessed: {}", guess);
        attempts = increment(attempts);

        //select arm based on comparing guess to immutable reference to 'secret_number'
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! You took {} guesses.", attempts);
                break;
            }
        }
    }
}

//--- unit tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_by_1() {
        assert_eq!(increment(20), 21);

        let n1: u32 = 0;
        assert_eq!(increment(n1), 0 + 1);
    }
}
