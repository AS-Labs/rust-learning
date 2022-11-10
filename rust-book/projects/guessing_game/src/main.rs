// bringing the input/output library into scope
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// create the main function that has the program
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //    println!("Secret: {secret_number}");

    loop {
        println!("Please input your guess: (1-100)");

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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
}
