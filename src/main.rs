use std::io;
use std::num;
use structopt::StructOpt;

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

#[derive(Debug)]
#[derive(StructOpt)]
struct Opt {
    integers: Vec<u64>
}

fn main() {
    let opt = Opt::from_args();
    println!("Args: {:#?}", opt);
    let numbers: Vec<u64>;
    if !opt.integers.is_empty() {
        numbers = opt.integers;
    } else {
        numbers = [get_number().unwrap()].to_vec();
    }

    let mut memoized_collatz = MemoizedCollatz::default();

    for number in numbers.iter() {
        println!("Finding path length for '{}'", {number});
        let path_length = memoized_collatz.get_path_length(*number);
        println!("Path length is '{}'", {path_length});
    }
}
