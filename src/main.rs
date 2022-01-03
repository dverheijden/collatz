use std::io;
use std::num;
use std::path::Path;
use std::time::Instant;

use indicatif::ProgressBar;
use memoized::MemoizedCollatz;
use naive::NaiveCollatz;
use structopt::StructOpt;

use crate::collatz::Collatz;
use crate::memoized::FileSerialization;

mod collatz;
pub mod memoized;
pub mod naive;

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
    graph_filepath: Option<String>,
    integers: Vec<u64>,
}

fn interative(mut integers: Vec<u64>, collatz: &mut impl Collatz) {
    if integers.is_empty() {
        integers = [get_number().unwrap()].to_vec();
    }

    for number in integers.iter() {
        let now = Instant::now();
        println!("Finding path length for '{}'", { number });
        let path_length = collatz.get_path_length(*number);
        println!(
            "Path length is '{}' ({}μs)",
            path_length,
            now.elapsed().as_micros()
        );
    }
}

fn euler(collatz: &mut impl Collatz) {
    let mut max_path_length = 0;
    let mut number_which_produces_largest_path = 0;
    let progress_bar = ProgressBar::new(1000000);
    let now = Instant::now();
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
        "Largest path: {}\nNumber which produced it: {}\nTime elapsed: {}s",
        max_path_length,
        number_which_produces_largest_path,
        now.elapsed().as_secs()
    );
}

fn naive_main(integers: Vec<u64>) {
    let mut collatz = NaiveCollatz {};
    if !integers.is_empty() {
        interative(integers, &mut collatz);
    } else {
        euler(&mut collatz);
    }
}

fn memoized_main(path: String, integers: Vec<u64>) {
    let mut collatz;
    if Path::new(&path).exists() {
        println!("Loading graph file");
        collatz = MemoizedCollatz::from_file(&path);
    } else {
        println!("Creating graph from scratch");
        collatz = MemoizedCollatz::default();
    }

    if !integers.is_empty() {
        interative(integers, &mut collatz);
    } else {
        euler(&mut collatz);
    }

    collatz.to_file(&path);
}

fn main() {
    let opt = Opt::from_args();
    if opt.graph_filepath.is_none() {
        naive_main(opt.integers)
    } else {
        memoized_main(opt.graph_filepath.unwrap(), opt.integers);
    }
}
