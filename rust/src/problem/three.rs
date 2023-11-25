use super::Problem;
use std::collections::BTreeSet;

pub const THREE: Problem = Problem {
    number: 3,
    problem_a: a,
    problem_a_output: None,
    problem_b: b,
    problem_b_output: None,
};
#[derive(Clone, Debug)]
struct BackPack {
    front: BTreeSet<u32>,
    back: BTreeSet<u32>,
    total: BTreeSet<u32>,
}
impl BackPack {
    fn from_line(line: &str) -> Self {
        let scores = line
            .chars()
            .map(|char| char_to_score(char))
            .collect::<Vec<_>>();

        let mut front = BTreeSet::new();
        let mut back = BTreeSet::new();
        let mut total = BTreeSet::new();
        for i in 0..scores.len() / 2 {
            front.insert(scores[i]);
            total.insert(scores[i]);
        }
        for i in scores.len() / 2..scores.len() {
            back.insert(scores[i]);
            total.insert(scores[i]);
        }

        Self { front, back, total }
    }
    fn get_common_score(&self) -> u32 {
        *(self.front.intersection(&self.back).next().unwrap())
    }
}
fn char_to_score(c: char) -> u32 {
    let mut dest = [0u8];
    c.encode_utf8(&mut dest);
    let dest = dest[0];
    if c.is_ascii_lowercase() {
        (dest - 0x60) as u32
    } else if c.is_ascii_uppercase() {
        (dest - 0x41 + 27) as u32
    } else {
        panic!("invalid ascii")
    }
}
fn a(input: &str) -> String {
    input
        .split_whitespace()
        .map(|line| BackPack::from_line(line).get_common_score())
        .sum::<u32>()
        .to_string()
}
fn b(input: &str) -> String {
    let backpacks = input
        .split_whitespace()
        .map(|line| BackPack::from_line(line))
        .collect::<Vec<_>>();
    (0..backpacks.len() / 3)
        .map(|i| {
            let intersection_set = backpacks[i * 3]
                .total
                .intersection(&backpacks[i * 3 + 1].total)
                .copied()
                .collect::<BTreeSet<_>>();
            intersection_set
                .intersection(&backpacks[i * 3 + 2].total)
                .next()
                .copied()
                .expect("no intersection")
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "157")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "70");
    }
}
