use std::io;

fn main() {
    	println!("Guess the number!");
	println!("Please input your guess.");

	let mut guess = String::new(); //mut means the variable is mutable
	io::stdin()
		.read_line(&mut guess) //& denotes the variable is used by reference
		.expect("Failed to read line");
	println!("You guessed: {guess}");
}
