use super::Problem;
use std::cmp::max;

pub const ONE: Problem = Problem {
    number: 1,
    problem_a: a,
    problem_a_output: None,
    problem_b: b,
    problem_b_output: None,
};
#[derive(Clone, Copy)]
struct Elf {
    calories: u32,
}
fn parse_input(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|elf| Elf {
            calories: elf
                .split_whitespace()
                .map(|food| food.parse::<u32>().expect("failed to parse"))
                .sum::<u32>(),
        })
        .collect()
}
fn a(input: &str) -> String {
    let elves = parse_input(input)
        .iter()
        .fold(0u32, |acc, x| max(acc, x.calories));
    format!("{}", elves)
}
fn b(input: &str) -> String {
    let mut elves = parse_input(input);
    elves.sort_by(|a, b| a.calories.cmp(&b.calories));
    let sum = elves[elves.len() - 1].calories
        + elves[elves.len() - 2].calories
        + elves[elves.len() - 3].calories;
    sum.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "24000")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "45000");
    }
}
