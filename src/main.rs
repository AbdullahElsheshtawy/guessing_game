use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut num_of_tries: i32 = 0;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if (guess > 100) || (guess < 1){
            continue;
        }
        else{

            println!("You Guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small");
                    num_of_tries += 1;
                }
                Ordering::Greater => {
                    println!("Too big");
                    num_of_tries += 1;
                }
                Ordering::Equal => {
                    println!("You win");
                    num_of_tries += 1;
                    println!("You Won in {num_of_tries} tries");
                    break;
                }
            }
        }   
    }
}
