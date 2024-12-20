use std::{env::current_exe, fs};

#[derive(Debug, Clone)]
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

#[derive(Default, Clone)]
struct MapPosition {
    is_obstacle: bool,
    already_visited_up: bool,
    already_visited_right: bool,
    already_visited_down: bool,
    already_visited_left: bool,
}

impl MapPosition {
    fn already_visited(&self) -> bool {
        self.already_visited_up
            || self.already_visited_down
            || self.already_visited_left
            || self.already_visited_right
    }
    fn already_visited_dir(&self, dir: &Direction) -> bool {
        match dir {
            Direction::Up => self.already_visited_up,
            Direction::Right => self.already_visited_right,
            Direction::Down => self.already_visited_down,
            Direction::Left => self.already_visited_left,
        }
    }
    fn set_already_visited(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.already_visited_up = true,
            Direction::Right => self.already_visited_right = true,
            Direction::Down => self.already_visited_down = true,
            Direction::Left => self.already_visited_left = true,
        }
    }
}

fn get_position(map: &mut [Vec<MapPosition>], x: i32, y: i32) -> Option<&mut MapPosition> {
    // Convert both coordinates to usize
    // Early return None if the conversion fails as that means we're going out of bounds
    // (negative index)
    let xr = usize::try_from(x).ok()?;
    let yr = usize::try_from(y).ok()?;
    // Get the position with the coordinates
    // If either coordinate is out of bounds, return None
    map.get_mut(yr)?.get_mut(xr)
}

fn check_for_obstacle(
    orig_map: &[Vec<MapPosition>],
    x: i32,
    y: i32,
    input_dir: &Direction,
) -> bool {
    // Simulate what would happen if there was an obstacle in front of us at this position
    // Clone the map to keep our own state just for this test
    let mut map = orig_map.to_vec().clone();
    let mut current_dir = input_dir.clone();
    let mut cur_x = x;
    let mut cur_y = y;
    let mut first_iter = true;
    loop {
        let next_x = cur_x + current_dir.to_offset().0;
        let next_y = cur_y + current_dir.to_offset().1;
        if let Some(pos) = get_position(&mut map,
            next_x,
            next_y
        ) {
            if first_iter {
                // Check if our test position has already been walked over
                // If so, that means an obstacle can't go here
                if pos.already_visited() {
                    return false;
                }
                if !pos.is_obstacle {
                    // Place our loop obstacle
                    pos.is_obstacle = true;
                }
                first_iter = false;
            }
            if pos.is_obstacle {
                pos.set_already_visited(&current_dir);
                current_dir = current_dir.turn_clockwise();
            } else if pos.already_visited_dir(&current_dir) {
                // If we ever end up in a spot we've already been facing a direction we've already faced at
                // that spot, we have a loop and the obstacle is valid
                return true;
            } else {
                pos.set_already_visited(&current_dir);
                cur_x = next_x;
                cur_y = next_y;
            }
        // otherwise, we went out of bounds with either x or y, so the obstacle did not cause a loop
        } else {
            return false;
        }
    }
}

#[derive(Debug)]
enum PatrolSection {
    Stayed(i32, i32),
    Left(i32),
}

fn distance_to_obstacle(
    map: &mut [Vec<MapPosition>],
    start_x: i32,
    start_y: i32,
    dir: &Direction,
    possible_obstacle_count: &mut i32,
) -> PatrolSection {
    let mut distance = 0;
    let mut unrepeated_distance = 0;
    let mut cur_x = start_x;
    let mut cur_y = start_y;
    loop {
        if check_for_obstacle(map, cur_x, cur_y, dir) {
            *possible_obstacle_count += 1;
        }
        cur_x += dir.to_offset().0;
        cur_y += dir.to_offset().1;
        if let Some(pos) = get_position(
            map,
            cur_x,
            cur_y
        ) {
            if pos.is_obstacle {
                return PatrolSection::Stayed(distance, unrepeated_distance);
            } else {
                if !pos.already_visited() {
                    pos.set_already_visited(dir);
                    unrepeated_distance += 1;
                }
                distance += 1;
            }
        // otherwise, we went out of bounds with either x or y, so return Left
        } else {
            return PatrolSection::Left(unrepeated_distance);
        }
    }
}

fn solve(raw_data: &str) {
    let mut cur_x: i32 = 0;
    let mut cur_y: i32 = 0;
    let mut cur_dir = Direction::Up;
    let mut map: Vec<Vec<MapPosition>> = raw_data
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, char)| {
                    let mut out = MapPosition::default();
                    match char {
                        '^' => {
                            cur_x = x.try_into().expect("array index to always convert to i32");
                            cur_y = y.try_into().expect("array index to always convert to i32");
                            // Starting position is always already visited
                            out.already_visited_up = true;
                        }
                        '#' => out.is_obstacle = true,
                        '.' => {}
                        c => panic!("Unexpected character '{}'", c),
                    }
                    out
                })
                .collect()
        })
        .collect();
    // distance starts at 1 since the starting location counts
    let mut unique_distance = 1;
    let mut obstacle_count = 0;
    loop {
        match distance_to_obstacle(&mut map, cur_x, cur_y, &cur_dir, &mut obstacle_count) {
            PatrolSection::Stayed(total_dis, unique_dis) => {
                unique_distance += unique_dis;
                // Modify our x and y by adding our offset multiplied by the distance travelled
                cur_x += cur_dir.to_offset().0 * total_dis;
                cur_y += cur_dir.to_offset().1 * total_dis;
                // Turn
                cur_dir = cur_dir.turn_clockwise();
            }
            PatrolSection::Left(unique_dis) => {
                unique_distance += unique_dis;
                break;
            }
        }
    }
    println!("Total distance patrolled is {}", unique_distance);
    println!("Obstacle count is {}", obstacle_count);
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day6input.txt").expect("Failed to read input file!");
    solve(&raw_data);
}
