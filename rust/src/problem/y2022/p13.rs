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
#[derive(Clone, Debug, PartialEq)]
enum ListItem {
    List(Vec<ListItem>),
    Int(u32),
}
impl ListItem {
    fn in_right_order(&self, rhs: &Self) -> bool {
        match self {
            ListItem::List(self_vec) => {
                println!("self is list: {:#?}", self);
                match rhs {
                    ListItem::List(other_vec) => {
                        let mut lhs_iter = self_vec.iter();
                        let mut rhs_iter = other_vec.iter();
                        loop {
                            let lhs = lhs_iter.next();
                            let rhs = rhs_iter.next();
                            if lhs.is_none() && rhs.is_some() {
                                return true;
                            } else if lhs.is_some() && rhs.is_none() {
                                return false;
                            } else if lhs.is_some() && rhs.is_some() {
                                continue;
                            } else if !lhs.unwrap().in_right_order(rhs.unwrap()) {
                                return false;
                            } else {
                                continue;
                            }
                        }
                    }
                    ListItem::Int(rhs_val) => {
                        self.in_right_order(&ListItem::List(vec![ListItem::Int(*rhs_val)]))
                    }
                }
            }
            ListItem::Int(lhs_val) => match rhs {
                ListItem::List(rhs_val) => {
                    Self::List(vec![Self::Int(*lhs_val)]).in_right_order(rhs)
                }
                ListItem::Int(rhs_val) => lhs_val <= rhs_val,
            },
        }
    }
    pub fn from_str(input: &str) -> Self {
        let mut chars_iter = input.chars().peekable();
        Self::from_iterator(&mut chars_iter).unwrap()
    }
    fn from_iterator(chars: &mut Peekable<Chars>) -> Option<Self> {
        let first_char = chars.next().unwrap();
        if first_char.is_numeric() {
            let mut out_str = first_char.to_string();
            loop {
                let next_is_number = chars.peek().map(|c| c.is_numeric()).unwrap_or(false);
                if next_is_number {
                    out_str.push(chars.next().unwrap());
                } else {
                    return Some(Self::Int(out_str.parse().unwrap()));
                }
            }
        } else {
            if first_char == '[' {
                println!("doing list");
                Some(Self::List(Self::from_list(chars)))
            } else if first_char == ',' {
                chars.next();
                Self::from_iterator(chars)
            } else if first_char == ']' {
                None
            } else {
                panic!("invalid char: {}", first_char)
            }
        }
    }
    fn from_list(chars: &mut Peekable<Chars>) -> Vec<Self> {
        let mut out_vec = Vec::new();
        loop {
            let next = *chars.peek().unwrap();
            if next == ']' {
                chars.next().unwrap();
                return out_vec;
            } else {
                if next == ',' {
                    chars.next();
                }

                if let Some(item) = Self::from_iterator(chars) {
                    out_vec.push(item)
                } else {
                    return out_vec;
                }
            }
        }
    }
}
fn a(input: &str) -> String {
    let lists = input
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
    println!("len outer vec {}", lists.len());
    for pair in lists.iter() {
        println!("length of pair: {}", pair.len());
        for item in pair.iter() {
            println!("{:#?}", item)
        }
    }
    lists
        .iter()
        .enumerate()
        .filter_map(|(index, v)| {
            if v[0].in_right_order(&v[1]) {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
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
    fn first_pair() {
        let left = ListItem::from_str("[1,1,3,1,1]");
        let right = ListItem::from_str("[1,1,5,1,1]");
        assert!(left.in_right_order(&right));
        assert!(!right.in_right_order(&left));
    }
    #[test]
    fn second_pair() {
        let left = ListItem::from_str("[[1],[2,3,4]]");
        let right = ListItem::from_str("[[1],4]");
        println!("left: {:#?}", left);
        println!("right: {:#?}", right);
        assert!(left.in_right_order(&right));
        assert!(!right.in_right_order(&left));
    }
    #[test]
    fn parse_one() {
        let s = "[1,1,3,1,1]";
        let l = ListItem::from_str("[1,1,3,1,1]");
        assert_eq!(
            l,
            ListItem::List(vec![
                ListItem::Int(1),
                ListItem::Int(1),
                ListItem::Int(3),
                ListItem::Int(1),
                ListItem::Int(1)
            ])
        )
    }
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
