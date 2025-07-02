use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secretNum = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secretNum);
    println!("Please input your guess!");

    let mut guess = String::new(); //let creates VAR, make guess mutable (immutable by defualt)
    io::stdin()
        .read_line(&mut guess) //call read line method, & = reference
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secretNum) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Winner!"),
    }
}
