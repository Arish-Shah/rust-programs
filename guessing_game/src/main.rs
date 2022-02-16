use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    
    loop {
        let mut guess = String::new();

        println!("Enter your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read user input");

        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Error(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too less!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
