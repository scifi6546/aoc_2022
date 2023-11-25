use super::Problem;
use std::cmp::{max, min};
pub const FOUR: Problem = Problem {
    number: 4,
    problem_a: a,
    problem_a_output: Some("503"),
    problem_b: b,
    problem_b_output: Some("827"),
};
#[derive(Copy, Clone, Debug)]
struct Range {
    start: u32,
    end: u32,
}
impl Range {
    fn from_str(input: &str) -> Self {
        let mut split = input.split("-");
        let start = split.next().unwrap().parse().unwrap();
        let end = split.next().unwrap().parse().unwrap();
        Self { start, end }
    }

    fn fully_contains(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }
    fn does_overlap(&self, other: &Self) -> bool {
        if self.end < other.start {
            false
        } else if self.start > other.end {
            false
        } else {
            true
        }
    }
}
fn a(input: &str) -> String {
    input
        .split_whitespace()
        .filter_map(|line| {
            let mut s = line.split(",");
            if let Some(s1) = s.next() {
                if let Some(s2) = s.next() {
                    Some((Range::from_str(s1), Range::from_str(s2)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(range_1, range_2)| {
            range_1.fully_contains(&range_2) || range_2.fully_contains(&range_1)
        })
        .filter_map(|contains| if contains { Some(1u32) } else { None })
        .sum::<u32>()
        .to_string()
}
fn b(input: &str) -> String {
    input
        .split_whitespace()
        .filter_map(|line| {
            let mut s = line.split(",");
            if let Some(s1) = s.next() {
                if let Some(s2) = s.next() {
                    Some((Range::from_str(s1), Range::from_str(s2)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(range_1, range_2)| range_1.does_overlap(&range_2))
        .filter_map(|contains| if contains { Some(1u32) } else { None })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "2")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "4");
    }
}
