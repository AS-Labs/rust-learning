// bringing the input/output library into scope
use std::io;

// create the main function that has the program
fn main() {
    println!("Guess the number");

    println!("Please input your guess:");

    // make the variable mutable with "mut" as by default it is immutable
    // :: indicates that new is an associated function of the String type whic returns a string
    // type
    let mut guess = String::new();

    io::stdin()
        // calls the read_line method from io library
        // &mut is store whatever is read as a reference to guess
        // references are immutable by default thats why & is associated with mut not guess
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
