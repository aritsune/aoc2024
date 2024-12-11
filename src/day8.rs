use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
struct Coordinates(usize, usize);

impl Hash for Coordinates {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Coordinates(x, y) = self;
        // I have no idea what this is doing!
        let hash_val = ((x + y) * (x + y + 1) / 2) + y;

        state.write_usize(hash_val);
    }
}

fn ascii_to_antennas_map(raw_data: &str) -> HashMap<char, Vec<Coordinates>> {
    let mut map = HashMap::<char, Vec<Coordinates>>::new();
    for (y, line) in raw_data.lines().filter(|l| !l.is_empty()).enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                map.entry(char).or_default().push(Coordinates(x, y));
            }
        }
    }
    map
}

fn all_pairs<T: PartialEq>(input: &[T]) -> impl Iterator<Item = (&T, &T)> {
    input
        .iter()
        // Iterate over every element, treating it as the left element
        .flat_map(|left| {
            // Iterate over every element again
            input.iter().filter_map(move |right| {
                // But skip over the element we're treating as the left
                if right == left {
                    return None;
                }
                Some((left, right))
            })
        })
}

fn check_coords(
    in_x: Option<usize>,
    in_y: Option<usize>,
    xmax: usize,
    ymax: usize,
) -> Option<Coordinates> {
    match (in_x, in_y) {
        // If either coord number returns None, we went under 0 and the pair is invalid
        (None, _) | (_, None) => None,
        // If either coord went above its respective maximum, we went out of bounds and the pair is
        // invalid
        (Some(x), _) if x > xmax => None,
        (_, Some(y)) if y > ymax => None,
        // In any other case, it's valid
        (Some(x), Some(y)) => Some(Coordinates(x, y)),
    }
}

fn process_coord_pair(
    left: &Coordinates,
    right: &Coordinates,
    xmax: usize,
    ymax: usize,
    resonate: bool,
) -> Vec<Coordinates> {
    let mut out_vec = Vec::<Coordinates>::new();
    // Get offset from the left coords to the right coords, converting to isize as we need negative
    // values
    let offset_x = isize::try_from(right.0).expect("usize -> isize conversion to always work")
        - isize::try_from(left.0).expect("usize -> isize conversion to always work");
    let offset_y = isize::try_from(right.1).expect("usize -> isize conversion to always work")
        - isize::try_from(left.1).expect("usize -> isize conversion to always work");
    // Checked add the offset back onto the right coords
    let mut out_x = right.0.checked_add_signed(offset_x);
    let mut out_y = right.1.checked_add_signed(offset_y);
    // Repeatedly add the offset to our current coordinates and check if they're valid, pushing
    // them to our result vector each time
    while let Some(coords) = check_coords(out_x, out_y, xmax, ymax) {
        out_vec.push(coords);
        // But for part 1, only do it once
        if !resonate {
            break;
        }
        out_x = coords.0.checked_add_signed(offset_x);
        out_y = coords.1.checked_add_signed(offset_y);
    }
    out_vec
}

fn get_valid_pairs(
    map: &HashMap<char, Vec<Coordinates>>,
    ymax: usize,
    xmax: usize,
    resonate: bool,
) -> HashSet<Coordinates> {
    HashSet::<Coordinates>::from_iter(map.values().flat_map(|coords_for_antenna| {
        all_pairs(coords_for_antenna)
            .flat_map(|(left, right)| process_coord_pair(left, right, xmax, ymax, resonate))
    }))
}

fn solve(raw_data: &str) {
    let ymax = raw_data.lines().filter(|l| !l.is_empty()).count() - 1;
    let xmax = raw_data
        .lines()
        .next()
        .expect("raw data to have at least one line")
        .len()
        - 1;
    let antenna_map = ascii_to_antennas_map(raw_data);
    let valid_pairs_1 = get_valid_pairs(&antenna_map, ymax, xmax, false);
    println!(
        "Number of unique antinode positions without resonation is {}",
        valid_pairs_1.len()
    );
    let valid_pairs_2 = get_valid_pairs(&antenna_map, ymax, xmax, true);
    println!(
        "Number of unique antinode positions with resonation is {}",
        valid_pairs_2.len()
    );
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day8test.txt").expect("Failed to read input file!");
    solve(&raw_data);
}
