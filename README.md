Programming a guessing name.

The program will genrate a random integer from 1 to 100. It will then prompt the player to enter a guess. After the guess is entered the program will indicate whether the guess is too low or too high. If the guess is correct the program will print a congratulatory message and exit. 

Generating a secret number

The secret number should be different everytime. We will use rand crate to get more functionality. Crate is a collection of Rust source code files. The rand crate is a library crate which contains code that is intended to be used in oher programs and can't be executed on its own. 

Before we write a code that uses rand, we need to modify Cargo.toml file to include rand crate as a dependancy.

Rng trait defines methods that random number generators implement 


Comparing the gues to the Secret number.

We add use statement bringing a type called std::cmp::ordering into the scope from the standard library. The cmp method compares two values and can be called on anything that can be compared 