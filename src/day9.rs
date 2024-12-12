use std::fs;

fn calc_checksum(raw_data: &str, allow_fragmentation: bool) -> usize {
    // Left cursor is the main cursor
    let left_cursor = raw_data.chars().enumerate();
    let mut left_id = 0;
    // The right ID should be equal to ceil(len/2) (i.e., length ignoring free blocks)
    let mut right_id = raw_data.len() / 2;
    // Right cursor is used to grab from when we're on an empty block in the left cursor
    let mut right_cursor = raw_data
        .chars()
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
        .rev()
        .map(|(i, char)| {
            (
                i,
                char.to_digit(10)
                    .expect("characters in input to always parse to u32")
                    .try_into()
                    .expect("u32 to always parse to usize"),
            )
        });
    let mut right_next = || {
        right_cursor
            .next()
            .expect("right cursor to never reach end")
    };
    let (mut right_index, mut right_value) = right_next();
    let mut output: usize = 0;
    let mut output_position = 0;
    for (left_index, left_char) in left_cursor {
        if left_index == right_index {
            // Exit case - the left cursor has entered the right half of the string
            // Before we exit we need to add on any remaining data on the right that hasn't been
            // consumed by the left cursor
            // There should only be one item on the right left
            output += left_id * (output_position..(output_position + right_value)).sum::<usize>();
            break;
        };
        // Determine if the block under the left cursor is free or used
        let is_used = left_index % 2 == 0;
        let mut left_size: usize = left_char
            .to_digit(10)
            .expect("characters in input to always parse to u32")
            .try_into()
            .expect("u32 to always parse to usize");
        if is_used {
            // If used, add the checksum value by multiplying our current id by all the numbers in
            // the position range represented by the blcok
            // i.e.
            // id = 2, position = 3, size = 2
            // 2 * (3 + 4)
            // same as (2 * 3) + (2 * 4)
            output += left_id * (output_position..(output_position + left_size)).sum::<usize>();
            left_id += 1;
            output_position += left_size;
        } else {
            // We need to take from the right as many blocks of data as we have free space on the left
            // Decrement left_size until it's 0
            while left_size > 0 {
                let right_is_used = right_index % 2 == 0;
                if right_is_used {
                    // Check if we allow fragmentation
                    // If not, and right_value is bigger than left_size (i.e. the file at right is
                    // bigger than the free space we have available), then skip the rest of this free space
                    if !allow_fragmentation && right_value > left_size {
                        break;
                    }
                    // Either take however much the right has available, or however much the left needs
                    // Whichever is lower
                    let right_grabbed_value = usize::min(right_value, left_size);
                    let added_value = right_id
                        * ((output_position..(output_position + right_grabbed_value))
                            .sum::<usize>());
                    output_position += right_grabbed_value;
                    output += added_value;
                    // These should never go past zero as right_grabbed_value is the lowest of the
                    // two
                    left_size -= right_grabbed_value;
                    right_value -= right_grabbed_value;
                    // If right_value is 0 we move to the next file on the right
                    // and keep taking from it if we still need data on the left
                    if right_value == 0 {
                        right_id -= 1;
                        (right_index, right_value) = right_next();
                    }
                } else {
                    // Skip unused space on the right entirely
                    (right_index, right_value) = right_next();
                }
            }
        }
    }
    output
}

fn solve(raw_data: &str) {
    let checksum_fragmented = calc_checksum(raw_data, true);
    println!("Checksum with fragmentation is {}", checksum_fragmented);
    let checksum_unfragmented = calc_checksum(raw_data, false);
    println!(
        "Checksum without fragmentation is {}",
        checksum_unfragmented
    );
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day9test.txt").expect("input file to exist");
    solve(raw_data.trim());
}
