use std::io;   // This is the io library from the standard library
use rand::Rng;  // This is the rand library from the standard library

fn main() { // main function is the entry point of the program
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);  // This generates a random number between 1 and 100

    println!("The secret number is: {}", secret_number); // This prints the secret number to the console"

    println!("Please enter your guess number");

    let mut guess = String::new();  // This a variable that is mutable and stores the user input. let is used to create the variable.
    // = tells rust we want to bind something to a variable. 

    // Receiving user input
    io::stdin().read_line(&mut guess) // Calls the stdin from io which enables us to read the input from the user.
        .expect("Failed to read line");  // Handles potential failure of reading the input from the user.

    println!("You guessed: {}", guess);
}

