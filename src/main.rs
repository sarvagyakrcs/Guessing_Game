use rand::{self, Rng};
use std::cmp;
use std::io;
use colored::*;
fn main() {
    println!("Welcome To Guessing Game");
    println!("Please Input Your Guess");

    loop {
        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1..101);

        io::stdin().read_line(&mut guess).expect("Cannot Guess");
        /*
            Breaking Down Line:8 For userInput
           let handle = io::stdin(); //This line creates a handle to the standard input (stdin) using the io module. The stdin() function returns a handle that can be used to read input from the console.
           let result = handle.read_line(&mut guess); //will return a value of `Result` enum.
           result.expect("Cannot Input"); //The `expect` method is called on the Result to handle the result. If the result is an error (Err), the expect method will panic and print the provided error message.
        */

        // convert guess to int
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed {}", guess);
        println!("The Secret Number is {}", secret_number);

        let answer = guess.cmp(&secret_number); //`cmp` returns an ordering enum that has 3 possible values i.e `less`, `equal`, `greater`.

        match answer {
            cmp::Ordering::Less => println!("{}", "Less".red()),
            cmp::Ordering::Equal => {
                println!("{}", "Equal".green());
                break;
            },
            cmp::Ordering::Greater => println!("{}","Greater".red()),
        }
    }
}
