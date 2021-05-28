use std::io;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Guess The Number!");

    println!("Please Input your guess!");

    let mut guess = String::new(); // mutable variable
    let immutable = "immutable"; // immutable variable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed : {}", guess);
}
