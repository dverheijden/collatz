use crate::collatz::{compute_next, Collatz};

pub struct NaiveCollatz {}

impl Collatz for NaiveCollatz {
    fn get_next(&mut self, number: u64) -> u64 {
        compute_next(number)
    }
}
