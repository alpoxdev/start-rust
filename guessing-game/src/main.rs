use rand::Rng;
use std::cmp::Ordering;
use std::io;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Guess The Number!");
    println!("Please Input your guess!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new(); // mutable variable
        let immutable = "immutable"; // immutable variable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less!"),
            Ordering::Greater => println!("Greater!"),
            Ordering::Equal => {
                println!("Equal!");
                break;
            }
        }
    }
}
