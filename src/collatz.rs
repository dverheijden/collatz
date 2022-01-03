pub fn compute_next(number: u64) -> u64 {
    if number <= 1 {
        return 1;
    };
    if number % 2 == 0 {
        number / 2
    } else {
        3 * number + 1
    }
}

pub(crate) trait Collatz {
    fn get_next(&mut self, number: u64) -> u64;
    fn get_path_length(&mut self, mut number: u64) -> u64 {
        let mut path_length = 0;
        while number != 1 {
            number = self.get_next(number);
            path_length += 1;
        }
        path_length
    }
}
