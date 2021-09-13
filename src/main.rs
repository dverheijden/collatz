use std::io;
use std::num;

mod collatz;
pub use crate::collatz::MemoizedCollatz;

fn get_number() -> Result<u64, num::ParseIntError> {
    println!("Input a positive number:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u64 = guess.trim().parse()?;
    Ok(guess)
}

fn main() {
    let mut memoized_collatz = MemoizedCollatz::default();

    let mut number = get_number().unwrap();
    println!("Finding path length for '{}'", {number});

    let mut path_length = 0;
    while number != 1 {
        number = memoized_collatz.get_next(number);
        path_length += 1;
    }
    println!("Path length is '{}'", {path_length});
}
