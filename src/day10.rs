use core::panic;
use std::{collections::HashSet, fmt::Debug, fs, ops::DerefMut};

fn todigit(i: &u8) -> u8 {
    i - 48
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn x(&self) -> isize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 0,
            Self::Left => -1,
        }
    }
    fn y(&self) -> isize {
        match self {
            Self::Up => -1,
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 0,
        }
    }
    fn parallels(&self) -> [Direction; 2] {
        match self {
            Self::Up | Self::Down => [Self::Left, Self::Right],
            Self::Left | Self::Right => [Self::Up, Self::Down],
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

struct TopographicMap<'mapstr> {
    data: &'mapstr [u8],
    width: usize,
    trailheads: Vec<Coordinates>,
}

impl<'mapstr> TopographicMap<'mapstr> {
    fn from_ascii(data: &'mapstr [u8]) -> Self {
        let mut width: usize = 0;
        let mut trailheads = Vec::<Coordinates>::new();
        let mut y = 0;
        for (i, charnum) in data.iter().enumerate() {
            // digits
            if todigit(charnum) == 0 {
                trailheads.push(Coordinates::new(i - (y * width) - y, y));
            // newline
            } else if *charnum == 10 {
                if width == 0 {
                    width = i;
                }
                y += 1;
            } else if *charnum < 48 || *charnum > 57 {
                // not a digit
                panic!("Invalid character in provided input: {}", charnum);
            }
        }
        Self {
            data,
            width,
            trailheads,
        }
    }

    fn get_at(&self, coords: &Coordinates) -> Option<&u8> {
        self.data.get(coords.y + coords.y * self.width + coords.x)
    }

    fn spread_search(
        &self,
        coords: &Coordinates,
        direction: &Direction,
        peak_set: &mut HashSet<Coordinates>,
    ) -> (usize, usize) {
        //println!("Going {:?} from {:?}", direction, coords);
        let mut score = 0;
        let mut rating = 0;
        let mut curcoords = coords.clone();
        let mut last_op: Option<&u8> = None;
        while let Some(digit) = self.get_at(&curcoords) {
            // this code only runs on second+ iterations as on the first last_op will be None
            if let Some(last) = last_op {
                if digit - last != 1 {
                    //println!(
                    //    "Breaking due to bad gradient: {}->{} -- {:?}",
                    //    todigit(last),
                    //    todigit(digit),
                    //    curcoords
                    //);
                    break;
                } else if todigit(digit) == 9 {
                    // only check for a 9 if we have a last value (i.e. not first iteration)
                    // AND we passed the gradient check
                    //println!("Found peak (9) at {:?}", &curcoords);
                    if !peak_set.contains(&curcoords) {
                        peak_set.insert(curcoords);
                        score += 1;
                    }
                    rating += 1;
                    break;
                } else {
                    //println!("->{:?} {}->{}", curcoords, todigit(last), todigit(digit));
                    let (rec_score, rec_rating) = self.get_score_and_rating(
                        &curcoords,
                        &direction
                            .parallels()
                            .into_iter()
                            .filter(|dir| {
                                !(curcoords.y == 0 && *dir == Direction::Up)
                                    && !(curcoords.x == 0 && *dir == Direction::Left)
                            })
                            .collect::<Vec<_>>(),
                        peak_set,
                    );
                    score += rec_score;
                    rating += rec_rating;
                }
            }
            last_op = Some(digit);
            if let (Some(x), Some(y)) = (
                curcoords.x.checked_add_signed(direction.x()),
                curcoords.y.checked_add_signed(direction.y()),
            ) {
                curcoords = Coordinates { x, y };
                if let Some(&d) = self.get_at(&curcoords) {
                    if d == 10 {
                        //println!("Breaking due to hitting x boundary");
                        break;
                    }
                }
            } else {
                //println!("Breaking due to hitting other boundary");
                break;
            }
        }
        (score, rating)
    }

    fn get_score_and_rating(
        &self,
        coords: &Coordinates,
        directions: &[Direction],
        peak_set: &mut HashSet<Coordinates>,
    ) -> (usize, usize) {
        directions
            .iter()
            .map(|direction| self.spread_search(coords, direction, peak_set))
            .fold((0, 0), |acc, (score, rating)| {
                (acc.0 + score, acc.1 + rating)
            })
    }
    fn get_total_score_and_rating(&self) -> (usize, usize) {
        self.trailheads
            .iter()
            .map(|head_coords| {
                let mut map = HashSet::<Coordinates>::new();
                self.get_score_and_rating(head_coords, &DIRECTIONS, &mut map)
            })
            .fold((0, 0), |acc, (score, rating)| {
                (acc.0 + score, acc.1 + rating)
            })
    }
}

pub fn solve(raw_data: &[u8]) {
    let map = TopographicMap::from_ascii(raw_data);
    let (total_score, total_rating) = map.get_total_score_and_rating();
    println!("Total score of trailheads is {}", total_score);
    println!("Total rating of trailheads is {}", total_rating);
}

pub fn solution() {
    let raw_data = fs::read("input/day10input.txt").expect("Couldn't read input file!");
    solve(&raw_data);
}
