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
use std::io::Write;

pub mod util;
pub mod guess;

fn main() {
    //track the number of guesses
    let mut attempts: u32 = 0;

    println!("Guess the number! (1-{})", guess::MAX_NUM);

    //generate a random number 1-100
    let secret_number = rand::thread_rng().gen_range(1..guess::MAX_NUM + 1);

    //string formatting (comment this line to hide the answer!)
    //println!("The secret number is: {}", secret_number);

    'game: loop {
        print!("Please input your guess: ");
        std::io::stdout().flush().unwrap();

        //create a new mutable string
        let mut guess = String::new();

        //accept user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        //ignore a non-number guess and ask for another guess
        //shadow the 'guess' identifier to keep name as new datatype
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //catch-all errors with '_'
        };
        
        //do not want the program to panic! here because the user should still keep playing
        if guess < 1 || guess > guess::MAX_NUM {
            println!("The secret number will be between 1 and {}.", guess::MAX_NUM);
            continue;
        }

        let guess: guess::Guess = guess::Guess::new(guess);

        println!("You guessed: {}", guess.value());
        
        attempts = util::increment(attempts);

        //select arm based on comparing guess to immutable reference to 'secret_number'
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! You took {} guesses.", attempts);
                //print some feedback based on number of attempts used
                match attempts {
                    0 => println!("Hacker!"),
                    1 => println!("Go play the lotto!"),
                    2 | 3 | 4 => println!("A guessing machine!"),
                    5 | 6 | 7 => println!("A little bit of luck goes a long way."),
                    8 => println!("The logically worst-case performance possible!"),
                    _ => println!("Do better next time!")
                }
                //exit the loop
                break 'game;
            }
        }
    }
}
