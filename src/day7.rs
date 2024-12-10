#[derive(Clone, Copy, PartialEq, Debug)]
enum Operator {
    Plus,
    Times,
}

fn next_operator<T: PartialEq + Copy>(cur: T, list: &Vec<T>) -> Option<T> {
    let idx = list.iter().position(|i| *i == cur)? + 1;
    if idx > list.len() - 1 {
        return None;
    };
    return Some(list[idx]);
}

fn generate_operator_permutations(
    n: usize,
    operators: Vec<Operator>,
) -> impl Iterator<Item = Vec<Operator>> {
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
            current[next_non_maxed_index] =
                next_operator(dbg!(current[next_non_maxed_index]), &operators)
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
            current[last_pos] = next_operator(current[last_pos], &operators)
                .expect("current[pos] to never be last_op");
            // Return our current state
            Some(current.clone())
        }
    })
}

pub fn solve(raw_data: &str) {}

pub fn solution() {
    let res = generate_operator_permutations(4, vec![Operator::Plus, Operator::Times])
        .collect::<Vec<_>>();
    println!("{:#?}", res);
    println!("{} items", res.len());
}
