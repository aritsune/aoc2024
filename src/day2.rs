use proptest::proptest;
use std::fs;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().expect("Could not parse number!"))
                .collect()
        })
        .collect::<Vec<_>>()
}

fn test_is_safe(test: &[i32], dampen: bool) -> bool {
    let mut increased = false;
    let mut decreased = false;
    // Skip the first item, we only want items where we have a previous item to compare to
    if dampen {
        return (0..test.len())
            .any(|i| test_is_safe(&[&test[0..i], &test[(i + 1)..test.len()]].concat(), false));
    }
    for (i, num) in test[1..].iter().enumerate() {
        let last = test[i];
        let diff = last - num;
        match diff {
            // No change or too large change - not safe
            0 | ..=-4 | 4.. => return false,

            // Decreased within bounds
            -3..0 => {
                // If we've previously increased, it's not safe
                if increased {
                    return false;
                } else {
                    decreased = true;
                }
            }
            1..=3 => {
                if decreased {
                    return false;
                } else {
                    increased = true;
                }
            }
        }
    }
    // If we didn't hit any of the unsafe conditions and return early, it's safe
    true
}

pub fn solve(raw_data: &str) {
    let data = parse_input(raw_data);
    let safe_count: i32 = data
        .iter()
        .filter(|t| !t.is_empty())
        .map(|t| test_is_safe(t, false))
        .map(|res| match res {
            true => 1,
            false => 0,
        })
        .sum();
    println!("{} tests are safe without dampener", safe_count);
    let safe_count_with_dampener: i32 = data
        .iter()
        .filter(|t| !t.is_empty())
        .map(|t| test_is_safe(t, true))
        .map(|res| match res {
            true => 1,
            false => 0,
        })
        .sum();
    println!("{} tests are safe with dampener", safe_count_with_dampener);
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day2input.txt").expect("Failed to read input file!");
    solve(&raw_data);
}

proptest! {
    #[test]
    fn it_works(raw_data in "(([0-9]{1, 6} )*[0-9]{1, 6}\\n)+") {
        solve(&raw_data);
    }
}
