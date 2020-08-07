use std::io;

use clap::App; 

fn main() { 
    App::new("doots")
       .get_matches();
	
	println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);    
}
