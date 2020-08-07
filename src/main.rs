use text_io::read;
use clap::App; 

fn main() { 
    App::new("doots")
       .get_matches();
	
	println!("Guess the number!");
    println!("Please input your guess.");

	let guess: i32 = read!();

    println!("You guessed: {}", guess);    
    if guess == 69 {
	    println!("Nice");    
    }
}
