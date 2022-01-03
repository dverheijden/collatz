use std::io;
use std::num;
use std::path::Path;

use indicatif::ProgressBar;
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
    integers: Vec<u64>,
}

fn interative(mut integers: Vec<u64>, mut collatz: MemoizedCollatz) -> MemoizedCollatz {
    if integers.is_empty() {
        integers = [get_number().unwrap()].to_vec();
    }

    for number in integers.iter() {
        println!("Finding path length for '{}'", { number });
        let path_length = collatz.get_path_length(*number);
        println!("Path length is '{}'", { path_length });
    }
    collatz
}

fn euler(mut collatz: MemoizedCollatz) -> MemoizedCollatz {
    let mut max_path_length = 0;
    let mut number_which_produces_largest_path = 0;
    let progress_bar = ProgressBar::new(1000000);
    for number in 0..1000000 {
        let path_length = collatz.get_path_length(number);
        if path_length > max_path_length {
            max_path_length = path_length;
            number_which_produces_largest_path = number;
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();
    println!(
        "Largest path: {}\nNumber which produced it:{}",
        max_path_length, number_which_produces_largest_path
    );
    collatz
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

    if !opt.integers.is_empty() {
        memoized_collatz = interative(opt.integers, memoized_collatz);
    } else {
        memoized_collatz = euler(memoized_collatz);
    }

    memoized_collatz.to_file(&opt.graph_filepath);
}
