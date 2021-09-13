use std::io;
use std::path::Path;
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

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "f", long)]
    graph_filepath: String,
    integers: Vec<u64>
}

fn main() {
    let opt = Opt::from_args();
    let mut memoized_collatz;
    if Path::new(&opt.graph_filepath).exists() {
        println!("Loading graph file");
        memoized_collatz = MemoizedCollatz::from_file(&opt.graph_filepath);
    } else {
        println!("Creating graph from scratch");
        memoized_collatz = MemoizedCollatz::default();
    }

    let numbers: Vec<u64>;
    if !opt.integers.is_empty() {
        numbers = opt.integers;
    } else {
        numbers = [get_number().unwrap()].to_vec();
    }

    for number in numbers.iter() {
        println!("Finding path length for '{}'", {number});
        let path_length = memoized_collatz.get_path_length(*number);
        println!("Path length is '{}'", {path_length});
    }

    memoized_collatz.to_file(&opt.graph_filepath);
}
