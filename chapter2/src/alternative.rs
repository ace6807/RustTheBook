use rand::Rng;
use std::io::Write;

fn main() {
    println!("Guess the number!");
    let random_number = rand::thread_rng().gen_range(1..=100);

    let mut score = 0;
    loop {
        score += 1;

        print!("Please input your guess: ");
        std::io::stdout().flush().unwrap();
    
        let mut guess = String::new();
        match std::io::stdin().read_line(&mut guess){
            Ok(_) => {},
            Err(_) => {
                println!("Error reading line");
                std::process::exit(1);
            },
        }   

        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                continue;
            },
        };

        
        match guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win! Your score is {score}");
                break;
            }
        }
    
    }

}
