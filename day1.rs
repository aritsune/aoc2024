use std::collections::HashMap;
use std::fs;

fn sort<T: Copy + Ord + std::fmt::Debug>(to_sort: &[T]) -> Vec<T> {
    // janky out of place insertion sort
    let mut output = vec![to_sort[0]];
    for (i, &item) in to_sort[1..].iter().enumerate() {
        // i is actually i - 1 due to the slice operation
        let mut j = i;
        let mut comp_item = output[j];
        loop {
            if item > comp_item {
                output.insert(j + 1, item);
                break;
            }
            if j == 0 {
                output.insert(0, item);
                break;
            }
            j -= 1;
            comp_item = output[j];
        }
    }
    output
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut one = Vec::<i32>::new();
    let mut two = Vec::<i32>::new();
    for line in input.lines() {
        let mut nums = line.split_whitespace();
        one.push(
            nums.next()
                .map(|i| i.parse().expect("Could not parse number!"))
                .expect("Could not find number!"),
        );
        two.push(
            nums.next()
                .map(|i| i.parse().expect("Could not parse number!"))
                .expect("Could not find number!"),
        );
    }
    (one, two)
}

fn get_total_distance(one: Vec<i32>, two: Vec<i32>) -> i32 {
    let mut total = 0;
    for (i, _) in one.iter().enumerate() {
        let (left, right) = (one[i], two[i]);
        if left > right {
            total += left - right;
        } else {
            total += right - left
        }
    }
    total
}

fn get_similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut count_map = HashMap::<i32, i32>::new();
    for num in right.iter() {
        let count = count_map.entry(*num).or_insert(0);
        *count += num;
    }
    let mut total = 0;
    for num in left.iter() {
        total += count_map.get(num).unwrap_or(&0);
    }
    total
}

fn main() {
    let raw_input = fs::read_to_string("day1input.txt").expect("Couldn't read input file!");
    let (left, right) = parse_input(&raw_input);
    let (left_sorted, right_sorted) = (sort(&left), sort(&right));
    let total_distance = get_total_distance(left_sorted, right_sorted);
    println!("Total distance is {:?}", total_distance);
    let similarity_score = get_similarity_score(left, right);
    println!("Similarity score is {:?}", similarity_score);
}
