use std::collections::{HashMap, HashSet};
use std::fs;

pub fn set_is_valid(set: &[u8], rules_map: &HashMap<u8, HashSet<u8>>) -> bool {
    for (i, num) in set.iter().enumerate().skip(1) {
        let previous = &set[..=i];
        match rules_map.get(num) {
            None => continue,
            Some(not_allowed) => {
                for prevnum in previous {
                    if not_allowed.contains(prevnum) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

pub fn fix_set(mut set: Vec<u8>, rules_map: &HashMap<u8, HashSet<u8>>) -> Vec<u8> {
    // Iterate over every number in set
    for i in 0..set.len() {
        match rules_map.get(&set[i]) {
            None => continue,
            Some(not_allowed) => {
                // Iterate over previous numbers
                for j in 0..i {
                    // If we run into any number that should be after set[i], swap them, then move
                    // on to the next i
                    if not_allowed.contains(&set[j]) {
                        set[j..=i].rotate_right(1);
                        break;
                    }
                }
            }
        }
    }
    set
}

pub fn solve(data: &str) {
    let mut rules_map: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut split: usize = 0;
    for (i, ordline) in data.lines().enumerate() {
        if ordline.is_empty() {
            split = i;
            break;
        };
        let (left, right) = ordline.split_at(2);
        let before: u8 = left
            .parse()
            .expect("left side number to be parseable into u8");
        let after: u8 = right[1..]
            .parse()
            .expect("right side number to be parseable into u8");
        // Add `after` to the list of numbers that cannot be before `before`
        let after_set = rules_map.entry(before).or_default();
        after_set.insert(after);
    }
    let mut valid_sets: Vec<Vec<u8>> = Vec::new();
    for updateline in data.lines().skip(split + 1) {
        if updateline.is_empty() {
            continue;
        }
        let updateset: Vec<u8> = updateline
            .split(',')
            .map(|strnum| {
                strnum
                    .parse::<u8>()
                    .expect("all numbers in input data to be parseable")
            })
            .collect();
        if set_is_valid(&updateset, &rules_map) {
            valid_sets.push(updateset);
        }
    }
    let answer: usize = valid_sets
        .into_iter()
        .map(|set| usize::from(set[set.len() / 2]))
        .sum();
    println!("Total of middle numbers of correct sets is {}", answer);
    let mut fixed_sets: Vec<Vec<u8>> = Vec::new();
    for updateline in data.lines().skip(split + 1) {
        if updateline.is_empty() {
            continue;
        }
        let mut updateset: Vec<u8> = updateline
            .split(',')
            .map(|strnum| {
                strnum
                    .parse::<u8>()
                    .expect("all numbers in input data to be parseable")
            })
            .collect();
        if !set_is_valid(&updateset, &rules_map) {
            updateset = fix_set(updateset, &rules_map);
            assert!(set_is_valid(&updateset, &rules_map));
            fixed_sets.push(updateset);
        }
    }
    let answer_two: usize = fixed_sets
        .into_iter()
        .map(|set| usize::from(set[set.len() / 2]))
        .sum();
    println!(
        "Total of middle numbers of fixed incorrect sets is {}",
        answer_two
    );
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day5input.txt").expect("Failed to read input file!");
    solve(&raw_data);
}
