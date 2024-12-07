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

#[derive(Default)]
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
) -> PatrolSection {
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
                _ => return PatrolSection::Left(unrepeated_distance),
            }
        };
        // if this is Some then there is a character at (x, y)
        if let Some(pos) = map.get_mut(y).and_then(|row| row.get_mut(x)) {
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
                        },
                        '#' => out.is_obstacle = true,
                        '.' => {},
                        c => panic!("Unexpected character '{}'", c),
                    } 
                    out
                })
                .collect()
        })
        .collect();
    // distance starts at 1 since the starting location counts
    let mut unique_distance = 1;
    loop {
        match distance_to_obstacle(&mut map, cur_x, cur_y, &cur_dir) {
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
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day6input.txt").expect("Failed to read input file!");
    solve(&raw_data);
}
