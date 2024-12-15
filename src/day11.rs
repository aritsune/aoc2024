use crate::util::count_digits;
use std::fs;

pub fn get_blink_splits(mut num: usize, blinks: usize) -> usize {
    if blinks == 0 {
        return 0;
    }
    let mut splits = 0;
    for i in 1..=blinks {
        let digits = count_digits(&num);
        if num == 0 {
            num = 1;
        } else if digits % 2 == 0 {
            splits += 1;
            let left = num / 10_usize.pow((digits / 2).try_into().unwrap());
            let right = num - left * 10_usize.pow((digits / 2).try_into().unwrap());
            let left_splits = get_blink_splits(left, blinks - i);
            splits += left_splits;
            let right_splits = get_blink_splits(right, blinks - i);
            splits += right_splits;
            break;
        } else {
            num *= 2024;
        }
    }
    splits
}

pub fn solve(raw_data: &str) {
    let stones: Vec<usize> = raw_data
        .trim()
        .split(' ')
        .map(|numstr| str::parse(numstr).expect("all input data to parse as usize"))
        .collect();
    const BLINK_COUNT: usize = 25;
    let part1len: usize = stones
        .iter()
        .map(|num| 1 + get_blink_splits(*num, BLINK_COUNT))
        .sum();
    println!(
        "Blinking {} times results in {} stones",
        BLINK_COUNT, part1len
    );
    const BLINK_COUNT_PART2: usize = 75;
    let part2len: usize = stones
        .iter()
        .map(|num| 1 + get_blink_splits(*num, BLINK_COUNT_PART2))
        .sum();
    println!(
        "Blinking {} times results in {} stones",
        BLINK_COUNT, part2len
    );
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day11input.txt").expect("Couldn't read input file!");
    solve(&raw_data);
}
