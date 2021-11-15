use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    // get a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");

    // loop forever
    loop {
        // read a line from prompt
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert the line to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // if the guess is correct, break out of the loop
                break;
            }
        }
    }
}
