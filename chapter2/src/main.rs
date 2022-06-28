use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn print_title(){
    println!("\nGuess the number!");
    println!("-----------------\n\n");
}

fn generate_winning_number() -> u32 {
    return rand::thread_rng().gen_range(1..5);
}

fn get_user_guess() -> String {

    print!("Enter your guess: ");
    io::stdout().flush().expect("Failed to flush buffer!");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess;
}

fn main() {
    print_title();
    let secret_number = generate_winning_number();
    
    loop {       
        let guess: u32 = match get_user_guess().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
