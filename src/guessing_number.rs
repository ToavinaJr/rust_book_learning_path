use std::{cmp::Ordering, io};
use rand::Rng;

pub fn play_game() {
    println!("Guess the integer number between [1; 100]!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        println!("You guessed: {}", guess);
        
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a integer!");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The number is {}", guess);
                break;
            },
        }
    }
}