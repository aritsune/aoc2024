pub fn count_digits(num: &usize) -> usize {
    (*num as f64).log(10.0).trunc() as usize + 1
}
