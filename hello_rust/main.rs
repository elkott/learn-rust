use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess a number!");

    let sec_num = rand::thread_rng().gen_range(1, 10);

    let mut solution_found: bool = false;
    while solution_found == false {
        println!("\nInput your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        print!("You guessed: {}", guess);

        let guess: u32 = guess.trim().parse().expect("Enter a number!");

        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You guessed it!!");
                solution_found = true;
            }
        }
    }
}
