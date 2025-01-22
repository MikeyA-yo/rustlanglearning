use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
        
        // println!("This is the secret number: {secret_number}");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
