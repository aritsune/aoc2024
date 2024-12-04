use std::{fs, usize};

struct WordSearch {
    vec: Vec<Vec<char>>,
}

static DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

fn dir_to_str(dir: (i32, i32)) -> String {
    match dir {
        (1, 0) => "Forward",
        (-1, 0) => "Backward",
        (0, 1) => "Downward",
        (0, -1) => "Upward",
        (1, 1) => "Down-right",
        (-1, 1) => "Down-left",
        (1, -1) => "Up-right",
        (-1, -1) => "Up-left",
        _ => "Bad direction",
    }
    .to_owned()
}

impl WordSearch {
    fn get_at(&self, x: usize, y: usize) -> Option<&char> {
        let line = self.vec.get(y)?;
        line.get(x)
    }
    fn is_at(&self, x: usize, y: usize, char: &char) -> bool {
        match self.get_at(x, y) {
            None => false,
            Some(i) => i == char,
        }
    }
    fn search_word(
        &self,
        startx: usize,
        starty: usize,
        word: &str,
        dir_filter: Vec<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        // returns number of directions that matched
        DIRECTIONS
            .iter()
            .filter(|dir| dir_filter.is_empty() || dir_filter.contains(dir))
            .filter_map(|(offx, offy)| {
                let mut curx = startx;
                let mut cury = starty;
                for (i, search_char) in word.chars().enumerate() {
                    if !self.is_at(curx, cury, &search_char) {
                        return None;
                    };
                    if i < word.len() - 1 {
                        let newx: usize = (i32::try_from(curx).ok()? + offx).try_into().ok()?;
                        let newy: usize = (i32::try_from(cury).ok()? + offy).try_into().ok()?;
                        curx = newx;
                        cury = newy;
                    }
                }
                Some((*offx, *offy))
            })
            .collect()
    }
}

impl From<&str> for WordSearch {
    fn from(value: &str) -> Self {
        WordSearch {
            vec: value.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

fn count_xmas(data: &str) {
    let wordsearch = WordSearch::from(data);
    let mut count = 0;
    for y in 0..wordsearch.vec.len() {
        // possible optimization: all lines are same length in input data
        for x in 0..(wordsearch.vec[y].len()) {
            if *wordsearch
                .get_at(x, y)
                .expect("solve to not iterate outside of arrays")
                != 'X'
            {
                continue;
            };
            count += wordsearch.search_word(x, y, "XMAS", Vec::new()).len();
        }
    }
    println!("XMAS count is {}", count);
}

fn count_ecks_mas(data: &str) {
    let wordsearch = WordSearch::from(data);
    let mut count = 0;
    for y in 0..wordsearch.vec.len() {
        // possible optimization: all lines are same length in input data
        for x in 0..(wordsearch.vec[y].len()) {
            if *wordsearch
                .get_at(x, y)
                .expect("solve to not iterate outside of arrays")
                != 'M'
            {
                continue;
            };
            let directions = wordsearch.search_word(
                x,
                y,
                "MAS",
                vec![
                    // Down-right (with a matching up-right)
                    (1, 1),
                    // Up-right (with a matching up-left)
                    (1, -1),
                    // Up-left (with a matching down-left)
                    (-1, -1),
                    // Down-left (with a matching down-right)
                    (-1, 1),
                ],
            );
            for dir in directions {
                let offset = match dir {
                    (1, 1) => (0, 2),
                    (1, -1) => (2, 0),
                    (-1, -1) => (0, -2),
                    (-1, 1) => (-2, 0),
                    _ => panic!("Unexpected direction"),
                };
                let other_m_x = match usize::try_from(i32::try_from(x).expect("any x to convert to i32") + offset.0) {
                    Ok(i) => i,
                    Err(_) => continue,
                };
                let other_m_y = match usize::try_from(i32::try_from(y).expect("any x to convert to i32")+ offset.1) {
                    Ok(i) => i,
                    Err(_) => continue,
                };
                if let Some('M') = wordsearch.get_at(other_m_x, other_m_y) {
                    let dirs_to_check = match dir {
                        (1, 1) => vec![(1, -1)],
                        (1, -1) => vec![(-1, -1)],
                        (-1, -1) => vec![(-1, 1)],
                        (-1, 1) => vec![(1, 1)],
                        _ => panic!("Unexpected direction"),
                    };
                    let other_m_directions =
                        wordsearch.search_word(other_m_x, other_m_y, "MAS", dirs_to_check);
                    match other_m_directions.len() {
                        0 => {}
                        1 => count += 1,
                        2.. => panic!("Somehow found more than one direction"),
                    }
                }
            }
        }
    }
    println!("X-MAS count is {}", count);
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day4input.txt").expect("Failed to read input file!");
    count_xmas(&raw_data);
    count_ecks_mas(&raw_data);
}

#[test]
fn it_works() {
    todo!();
}
