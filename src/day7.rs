use std::fs;

use proptest::proptest;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Operator {
    Plus,
    Times,
    Concat,
}

fn next_operator<T: PartialEq + Copy>(cur: T, list: &[T]) -> Option<T> {
    let idx = list.iter().position(|i| *i == cur)? + 1;
    if idx > list.len() - 1 {
        return None;
    };
    Some(list[idx])
}

fn generate_operator_permutations(
    n: usize,
    operators: &'_ [Operator],
) -> impl Iterator<Item = Vec<Operator>> + '_ {
    let mut first_iter = true;
    let first_op = operators[0];
    let last_op = operators[operators.len() - 1];
    let mut current = vec![first_op; n];

    // Start at the right
    let last_pos = n - 1;
    std::iter::from_fn(move || {
        // Don't modify the first iteration and just send it out
        if first_iter {
            first_iter = false;
            return Some(current.clone());
        }

        if current[last_pos] == last_op {
            // If our current item is the last operator in the operators vector:
            // Find the rightmost item that is not maxed out
            // Early return: If there is no item that is not maxed out, the last iteration was the
            // last, so end the iteration with None
            let next_non_maxed_index = current
                .iter()
                .enumerate()
                .rev()
                .find(|(_, op)| **op != last_op)
                .map(|(i, _)| i)?;
            current[next_non_maxed_index] = next_operator(current[next_non_maxed_index], operators)
                .expect("next_op of next_non_maxed item to never be None");
            // Reset every item to the right of next_non_maxed_index to first operator
            current
                .iter_mut()
                .skip(next_non_maxed_index + 1)
                .for_each(|i| *i = first_op);
            // Return our current state
            Some(current.clone())
        } else {
            // Otherwise, just increment the operator at pos
            // Get the next operator from the list
            current[last_pos] = next_operator(current[last_pos], operators)
                .expect("current[pos] to never be last_op");
            // Return our current state
            Some(current.clone())
        }
    })
}

struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn parse_raw_data(raw_data: &'_ str) -> impl Iterator<Item = Equation> + '_ {
    raw_data
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| l.split_once(':'))
        .map(|(test_val_string, numbers_string)| Equation {
            test_value: str::parse(test_val_string).expect("test_val_string to be a valid i64"),
            numbers: numbers_string
                .trim()
                .split(' ')
                .map(|num_str| str::parse(num_str).expect("all numbers after : to be valid"))
                .collect::<Vec<_>>(),
        })
}

fn number_concat(left: i64, right: i64) -> i64 {
    let right_digits = (right as f64).log(10.0).trunc() as u32 + 1;
    (left * 10_i64.pow(right_digits)) + right
}

fn test_equation(equation: &Equation, operators: &[Operator]) -> bool {
    if equation.numbers.is_empty() {
        return false;
    }
    let mut op_permutations = generate_operator_permutations(equation.numbers.len() - 1, operators);
    // Iterate over every permutation of len - 1 operators
    // return true as soon as we find any match
    op_permutations.any(|perm| {
        equation
            // Iterate over the numbers in the equation
            .numbers
            .iter()
            // Skip the first number, as we'll use it as the init value when folding
            .skip(1)
            // Zip (combine) the number iterator with an iterator over the operators of this permutation
            // Since we skipped the first value of `numbers`, these iterators should both be the same length
            // (since perm.len() == numbers.len() - 1)
            .zip(perm)
            // Fold (reduce) by starting with the first number and then doing `acc = acc [operator] next` until we're out of iterations
            .fold(equation.numbers[0], |acc, (n, operator)| match operator {
                Operator::Plus => acc + n,
                Operator::Times => acc * n,
                Operator::Concat => number_concat(acc, *n),
            })
            // Compare the result of our fold with our test value
            == equation.test_value
    })
}

pub fn solve(raw_data: &str) {
    let total_sum_p1: i64 = parse_raw_data(raw_data)
        .filter_map(
            |eq| match test_equation(&eq, &[Operator::Plus, Operator::Times]) {
                true => Some(eq.test_value),
                false => None,
            },
        )
        .sum();
    println!(
        "Total sum of valid equations for part 1 is {}",
        total_sum_p1
    );
    let total_sum_p2: i64 = parse_raw_data(raw_data)
        .filter_map(|eq| {
            match test_equation(&eq, &[Operator::Plus, Operator::Times, Operator::Concat]) {
                true => Some(eq.test_value),
                false => None,
            }
        })
        .sum();
    println!(
        "Total sum of valid equations for part 2 is {}",
        total_sum_p2
    );
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day7input.txt").expect("Failed to read input file!");
    solve(&raw_data);
}

proptest! {
    #[test]
    fn test_num_concat(left in 0..10000000_i32, right in 0..10000000_i32) {
        let slow = str::parse::<i64>(&(left.to_string() + &right.to_string())).expect("slow string concat to work");
        assert_eq!(
            number_concat(left.into(), right.into()), slow
        )
    }
}
