use std::{fmt::Debug, fs};

fn calc_checksum_fragmented(raw_data: &str) -> usize {
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
            while left_size > 0 {
                let right_is_used = right_index % 2 == 0;
                if right_is_used {
                    // Check if we allow fragmentation
                    // If not, and right_value is bigger than left_size (i.e. the file at right is
                    // bigger than the free space we have available), then skip the rest of this free space
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

#[derive(Clone, Copy)]
enum Block {
    Free(usize),
    Used(usize, usize),
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free(size) => f.write_str(&(0..*size).map(|_| '.').collect::<String>()),
            Self::Used(id, size) => {
                f.write_str(&(0..*size).map(|_| id.to_string()).collect::<String>())
            }
        }
    }
}

fn calc_checksum_nofrag(raw_data: &str) -> usize {
    let mut blocks = raw_data
        .chars()
        .enumerate()
        .filter_map(|(i, char)| {
            let size = char
                .to_digit(10)
                .expect("characters in input to always parse to u32")
                .try_into()
                .expect("u32 to always parse to usize");
            if size == 0 {
                None
            } else if i % 2 == 0 {
                Some(Block::Used(i / 2, size))
            } else {
                Some(Block::Free(size))
            }
        })
        .collect::<Vec<_>>();
    //
    let mut initial_length = blocks.len();
    let mut i = blocks.len() - 1;
    while let Some(item) = blocks.get(i).copied() {
        if let Block::Used(iid, isize) = item {
            // find leftmost free block that fits
            let destination_slot = (0..i)
                .filter_map(|i| match blocks.get(i) {
                    Some(Block::Free(jsize)) => Some((i, *jsize)),
                    Some(Block::Used(_, _)) => None,
                    None => None,
                })
                .find(|(_, jsize)| *jsize >= isize);
            if let Some((destination_i, destination_size)) = destination_slot {
                if isize < destination_size {
                    blocks.remove(i);
                    blocks.remove(destination_i);
                    blocks.insert(destination_i, Block::Free(destination_size - isize));
                    blocks.insert(destination_i, Block::Used(iid, isize));
                    blocks.insert(i + 1, Block::Free(isize));
                } else {
                    blocks.swap(i, destination_i);
                }
            }
        }
        i -= 1;
    }
    let mut pos = 0;
    blocks
        .iter()
        .map(|block| match block {
            Block::Free(size) => {
                pos += size;
                0
            }
            Block::Used(id, size) => {
                let result = id * (pos..pos + size).sum::<usize>();
                pos += size;
                result
            }
        })
        .sum()
}

fn solve(raw_data: &str) {
    let checksum_fragmented = calc_checksum_fragmented(raw_data);
    println!("Checksum with fragmentation is {}", checksum_fragmented);
    let checksum_unfragmented = calc_checksum_nofrag(raw_data);
    println!(
        "Checksum without fragmentation is {}",
        checksum_unfragmented
    );
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day9input.txt").expect("input file to exist");
    solve(raw_data.trim());
}
