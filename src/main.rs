use std::io;   // This is the io library from the standard library

fn main() { // main function is the entry point of the program
    println!("Guess the number!");

    println!("Please enter your guess number");

    let mut guess = String::new();  // This a variable that is mutable and stores the user input. let is used to create the variable.
    // = tells rust we want to bind something to a variable. 

    // Receiving user input
    io::stdin().read_line(&mut guess) // Calls the stdin from io which enables us to read the input from the user.
        .expect("Failed to read line");  // Handles potential failure of reading the input from the user.

    println!("You guessed: {}", guess);
}
