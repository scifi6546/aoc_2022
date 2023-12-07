use crate::problem::Problem;
use std::collections::{BTreeMap, BTreeSet};

pub const P_03: Problem = Problem {
    number: 3,
    problem_a: a,
    print_problem_a_output: true,
    problem_a_output: Some("525181"),
    problem_b: b,
    problem_b_output: Some("84289137"),
    print_problem_b_output: true,
};
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn get_neighbors(&self) -> Vec<Self> {
        vec![
            Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}
#[derive(Clone, Debug)]
struct Grid {
    numbers: Vec<(String, Position)>,
    symbols: BTreeMap<Position, char>,
}
impl Grid {
    fn from_str(input: &str) -> Self {
        let chars = input
            .lines()
            .map(|s| s.trim())
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        (
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            c,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut numbers = Vec::new();
        let mut symbols = BTreeMap::new();
        for line in chars.iter() {
            let mut number_string = String::new();
            let mut number_pos = None;
            for (pos, char) in line.iter() {
                if char.is_numeric() {
                    if number_string.is_empty() {
                        number_pos = Some(*pos);
                    }
                    number_string.push(*char);
                } else {
                    if *char != '.' {
                        symbols.insert(*pos, *char);
                    }
                    if !number_string.is_empty() {
                        numbers.push((number_string.clone(), number_pos.unwrap()));
                        number_string.clear();
                    }
                }
            }
            if !number_string.is_empty() {
                numbers.push((number_string.clone(), number_pos.unwrap()));
                number_string.clear();
            }
        }

        Self { numbers, symbols }
    }
}
fn a(input: &str) -> String {
    let g = Grid::from_str(input);

    g.numbers
        .iter()
        .map(|(number, position)| {
            let has_neighboring_symbol = number
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    Position {
                        x: position.x + i as i32,
                        y: position.y,
                    }
                    .get_neighbors()
                    .iter()
                    .filter(|pos| g.symbols.contains_key(pos))
                    .count()
                        > 0
                })
                .fold(false, |acc, x| acc || x);
            if has_neighboring_symbol {
                number.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

fn b(input: &str) -> String {
    let g = Grid::from_str(input);
    let numbers = g
        .numbers
        .iter()
        .enumerate()
        .flat_map(|(string_id, (num_string, pos))| {
            num_string.chars().enumerate().map(move |(i, b)| {
                (
                    Position {
                        x: pos.x + i as i32,
                        y: pos.y,
                    },
                    (string_id.clone(), num_string.parse::<u32>().ok().unwrap()),
                )
            })
        })
        .collect::<BTreeMap<_, _>>();
    g.symbols
        .iter()
        .filter(|(pos, char)| **char == '*')
        .map(|(pos, _char)| {
            let neighboring_numbers = pos
                .get_neighbors()
                .iter()
                .filter_map(|neighbor_pos| numbers.get(neighbor_pos))
                .copied()
                .collect::<BTreeMap<_, _>>();
            if neighboring_numbers.len() == 2 {
                neighboring_numbers.iter().fold(1, |acc, (_id, x)| acc * x)
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    #[test]
    fn test_a() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "4361")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "467835");
    }
}
