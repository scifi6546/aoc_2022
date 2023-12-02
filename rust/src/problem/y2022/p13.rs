use super::Problem;
use std::{iter::Peekable, str::Chars};

pub const P_13: Problem = Problem {
    number: 13,
    problem_a: a,
    problem_a_output: None,
    print_problem_a_output: true,
    problem_b: b,
    problem_b_output: None,
    print_problem_b_output: true,
};
#[derive(Debug)]
enum ListItem {
    List(Vec<ListItem>),
    Int(u32),
}
impl ListItem {
    pub fn from_str(input: &str) -> Self {
        let mut chars_iter = input.chars().peekable();
        Self::from_iterator(&mut chars_iter).unwrap()
    }
    fn from_iterator(chars: &mut Peekable<Chars>) -> Option<Self> {
        let first_char = chars.next().unwrap();
        if first_char.is_numeric() {
            todo!("handle numeric!")
        } else {
            if first_char == '[' {
                Some(Self::List(Self::from_list(chars)))
            } else {
                panic!("invalid")
            }
        }
    }
    fn from_list(chars: &mut Peekable<Chars>) -> Vec<Self> {
        todo!()
    }
}
fn a(input: &str) -> String {
    let t = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(|line| ListItem::from_str(line))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("{:#?}", t);
    String::new()
}
fn b(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
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
