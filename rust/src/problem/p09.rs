use super::Problem;
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
    ops::Neg,
};

pub const P_09: Problem = Problem {
    number: 9,
    problem_a: a,
    problem_a_output: Some("6337"),
    problem_b: b,
    problem_b_output: Some("2455"),
};
#[derive(Clone, Copy, Debug)]
struct Step {
    delta_x: i32,
    delta_y: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn apply_step(&self, head: &Position) -> Self {
        let delta = *head - *self;
        return match delta {
            Position { x: -2, y: -1 } => Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position { x: -2, y: 0 } => Position {
                x: self.x - 1,
                y: self.y,
            },
            Position { x: -2, y: 1 } => Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Position { x: -1, y: -2 } => Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position { x: -1, y: -1 } => *self,
            Position { x: -1, y: 0 } => *self,
            Position { x: -1, y: 1 } => *self,
            Position { x: -1, y: 2 } => Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Position { x: 0, y: -2 } => Position {
                x: self.x,
                y: self.y - 1,
            },
            Position { x: 0, y: -1 } => *self,
            Position { x: 0, y: 0 } => *self,
            Position { x: 0, y: 1 } => *self,
            Position { x: 0, y: 2 } => Position {
                x: self.x,
                y: self.y + 1,
            },
            Position { x: 1, y: -2 } => Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Position { x: 1, y: -1 } => *self,
            Position { x: 1, y: 0 } => *self,
            Position { x: 1, y: 1 } => *self,
            Position { x: 1, y: 2 } => Position {
                x: self.x + 1,
                y: self.y + 1,
            },
            Position { x: 2, y: -1 } => Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Position { x: 2, y: 0 } => Position {
                x: self.x + 1,
                y: self.y,
            },
            Position { x: 2, y: 1 } => Position {
                x: self.x + 1,
                y: self.y + 1,
            },
            _ => panic!("unsupported delta: {}", delta),
        };
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Step {
    fn from_str(input: &str) -> Option<Self> {
        let input_chars = input.split_whitespace().take(2).collect::<Vec<_>>();

        if input_chars.len() == 2 {
            match input_chars[0] {
                "L" => Some(Self {
                    delta_x: input_chars[1].parse::<i32>().unwrap().neg(),
                    delta_y: 0,
                }),
                "R" => Some(Self {
                    delta_x: input_chars[1].parse::<i32>().unwrap(),
                    delta_y: 0,
                }),
                "U" => Some(Self {
                    delta_x: 0,
                    delta_y: input_chars[1].parse::<i32>().unwrap().neg(),
                }),
                "D" => Some(Self {
                    delta_x: 0,
                    delta_y: input_chars[1].parse::<i32>().unwrap(),
                }),
                _ => panic!("invalid direction"),
            }
        } else {
            None
        }
    }
}
fn a(input: &str) -> String {
    let steps = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| Step::from_str(line))
        .collect::<Vec<_>>();

    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    let mut visited_position: BTreeSet<Position> = BTreeSet::new();
    visited_position.insert(tail_position);
    for step in steps.iter() {
        if step.delta_x != 0 {
            let direction = if step.delta_x.is_positive() { 1 } else { -1 };
            for _x in 0..step.delta_x.abs() {
                head_position.x += direction;
                tail_position = tail_position.apply_step(&head_position);

                visited_position.insert(tail_position);
            }

            // handle move x
        } else if step.delta_y != 0 {
            let direction = if step.delta_y.is_positive() { 1 } else { -1 };
            for _y in 0..step.delta_y.abs() {
                head_position.y += direction;
                tail_position = tail_position.apply_step(&head_position);

                visited_position.insert(tail_position);
            }

            //handle move y
        } else {
            panic!("must move")
        }
    }
    visited_position.len().to_string()
}
fn b(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "13")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "");
    }
}
