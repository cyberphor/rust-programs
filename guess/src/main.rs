use std::{error, io};

fn main() {
    // print a message to the console
    println!("Guess the number");

    // declare a mutable variable, of the String type, called "guess"
    let mut guess: String;

    // initialize the "guess" variable by assigning a String object to it
    guess = String::new();

    // declare an immutable variable, of the Stdin type, called "keyboard"
    let keyboard: io::Stdin;

    // initialize the "keyboard" variable by assigning a Stdin object to it
    keyboard = io::stdin();

    // declare a immutable variable, of the Result type, called "result"
    // the Result enum will either be a usize or Error generic (from the "io" module)
    let result: Result<usize, io::Error>;

    // using the "keyboard" object's read_line() method,
    // read a line of input and 
    // save it to append it to the "guess" variable's buffer, then
    // save the result of this operaton to the "result" variable
    result = keyboard.read_line(&mut guess);

    // print the result of the previous operation
    println!("Result: {:?}", result);

    // print what was entered as input to the console
    println!("You guessed {}", guess);
}
