use std::fs;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
    fn turn_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug)]
enum PatrolStep {
    Stayed(i32, i32),
    Left(i32),
}

fn distance_to_obstacle(
    map: &mut [Vec<char>],
    start_x: i32,
    start_y: i32,
    dir: &Direction,
) -> PatrolStep {
    let mut distance = 0;
    let mut unrepeated_distance = 0;
    let mut cur_x = start_x;
    let mut cur_y = start_y;
    loop {
        cur_x += dir.to_offset().0;
        cur_y += dir.to_offset().1;
        // Convert cur_x, cur_y to usize so we can index into it
        // Make sure they're both positive while we're at it
        let (x, y) = {
            let xr = usize::try_from(cur_x).ok();
            let yr = usize::try_from(cur_y).ok();
            match (xr, yr) {
                // If both are Some then they're both positive
                (Some(x), Some(y)) => (x, y),
                // Otherwise, one of them is negative, we're out of bounds, so return Left from the
                // main function
                _ => return PatrolStep::Left(unrepeated_distance),
            }
        };
        // if this is Some then there is a character at (x, y)
        if let Some(char) = map.get_mut(y).and_then(|row| row.get_mut(x)) {
            if *char == '#' {
                return PatrolStep::Stayed(distance, unrepeated_distance);
            } else {
                if *char != 'X' {
                    *char = 'X';
                    unrepeated_distance += 1;
                }
                distance += 1;
            }
        // otherwise, we went out of bounds with either x or y, so return Left
        } else {
            return PatrolStep::Left(unrepeated_distance);
        }
    }
}

fn solve(raw_data: &str) {
    let mut cur_x: i32 = 0;
    let mut cur_y: i32 = 0;
    let mut cur_dir = Direction::Up;
    let mut map: Vec<Vec<char>> = raw_data
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, l)| {
                    if l == '^' {
                        cur_x = x.try_into().expect("array index to always convert to i32");
                        cur_y = y.try_into().expect("array index to always convert to i32");
                        // Starting position is always an X
                        return 'X';
                    }
                    l
                })
                .collect()
        })
        .collect();
    // distance starts at 1 since the starting location counts
    let mut unique_distance = 1;
    loop {
        match distance_to_obstacle(&mut map, cur_x, cur_y, &cur_dir) {
            PatrolStep::Stayed(total_dis, unique_dis) => {
                unique_distance += unique_dis;
                // Modify our x and y by adding our offset multiplied by the distance travelled
                cur_x += cur_dir.to_offset().0 * total_dis;
                cur_y += cur_dir.to_offset().1 * total_dis;
                // Turn
                cur_dir = cur_dir.turn_clockwise();
            }
            PatrolStep::Left(unique_dis) => {
                unique_distance += unique_dis;
                break;
            }
        }
    }
    println!("Total distance patrolled is {}", unique_distance);
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day6input.txt").expect("Failed to read input file!");
    solve(&raw_data);
}
