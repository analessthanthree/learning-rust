use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    loop {

        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You didn't type a number, silly!");
                    continue;
                },
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }
        }

        let mut play_again = String::new();
        println!("Play again? (y/n) ");
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        match play_again.as_str().trim() {
            "y" => {
                    println!("Good luck!");
                    continue;
            },
            _ => {
                    println!("What's the matter? Scared?");
                    break;
            },
        }

    }
        println!("Thanks for playing!")
}
