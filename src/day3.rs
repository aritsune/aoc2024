use std::fs;

enum MulState {
    Seeking,
    M,
    U,
    L,
    FirstNumber(Vec<char>),
    SecondNumber(Vec<char>, Vec<char>),
}

enum ConditionalState {
    Seeking,
    D,
    O,
    N,
    Apos,
    T,
    DoParen,
    DontParen,
}

pub fn solve(raw_data: &str, use_conditionals: bool) -> i32 {
    let mut total = 0;
    let mut mul_state = MulState::Seeking;
    let mut cond_state = ConditionalState::Seeking;
    let mut mul_allowed = true;

    for char in raw_data.chars() {
        if use_conditionals {
            match cond_state {
                ConditionalState::Seeking => if char == 'd' { cond_state = ConditionalState::D },
                ConditionalState::D => match char {
                    'o' => cond_state = ConditionalState::O,
                    _ => cond_state = ConditionalState::Seeking,
                },
                ConditionalState::O => match char {
                    'n' => cond_state = ConditionalState::N,
                    '(' => cond_state = ConditionalState::DoParen,
                    _ => cond_state = ConditionalState::Seeking,
                },
                ConditionalState::N => match char {
                    '\'' => cond_state = ConditionalState::Apos,
                    _ => cond_state = ConditionalState::Seeking,
                },
                ConditionalState::Apos => match char {
                    't' => cond_state = ConditionalState::T,
                    _ => cond_state = ConditionalState::Seeking,
                },
                ConditionalState::T => match char {
                    '(' => cond_state = ConditionalState::DontParen,
                    _ => cond_state = ConditionalState::Seeking,
                },
                ConditionalState::DoParen => match char {
                    ')' => {
                        mul_allowed = true;
                        cond_state = ConditionalState::Seeking;
                    },
                    _ => {
                        cond_state = ConditionalState::Seeking;
                    }
                },
                ConditionalState::DontParen => match char {
                    ')' => {
                        mul_allowed = false;
                        cond_state = ConditionalState::Seeking;
                    },
                    _ => {
                        cond_state = ConditionalState::Seeking;
                    }
                } 
            }
        }
        match mul_state {
            MulState::Seeking => if char == 'm' { mul_state = MulState::M },
            MulState::M => match char {
                'm' => {}
                'u' => {
                    mul_state = MulState::U;
                }
                _ => {
                    mul_state = MulState::Seeking;
                }
            },
            MulState::U => match char {
                'l' => {
                    mul_state = MulState::L;
                }
                _ => {
                    mul_state = MulState::Seeking;
                }
            },
            MulState::L => match char {
                '(' => {
                    mul_state = MulState::FirstNumber(Vec::new());
                }
                _ => {
                    mul_state = MulState::Seeking;
                }
            },
            MulState::FirstNumber(ref mut acc) => match char {
                '0'..='9' => {
                    acc.push(char);
                }
                ',' => {
                    mul_state = MulState::SecondNumber(acc.to_vec(), Vec::new());
                }
                _ => {
                    mul_state = MulState::Seeking;
                }
            },
            MulState::SecondNumber(ref num1acc, ref mut num2acc) => match char {
                '0'..='9' => {
                    num2acc.push(char);
                }
                ')' => {
                    if mul_allowed {
                        let num1: i32 = num1acc
                            .iter()
                            .collect::<String>()
                            .parse()
                            .expect("Failed to parse string for num1!");
                        let num2: i32 = num2acc
                            .iter()
                            .collect::<String>()
                            .parse()
                            .expect("Failed to parse string for num2!");
                        total += num1 * num2;
                        mul_state = MulState::Seeking;
                    }
                }
                _ => {
                    mul_state = MulState::Seeking;
                }
            },
        }
    }
    total
}

pub fn solution() {
    let raw_data = fs::read_to_string("input/day3input.txt").expect("Failed to read input file!");
    println!("Solution without conditionals is {}", solve(&raw_data, false));
    println!("Solution with conditionals is {}", solve(&raw_data, true));
}

#[test]
fn it_works() {
    let data1 = "asdhshsum(123,281)";
    solve(data1, false);
}
