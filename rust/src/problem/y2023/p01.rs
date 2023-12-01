use crate::problem::Problem;
use std::cmp::max;

pub const P_01: Problem = Problem {
    number: 1,
    problem_a: a,
    print_problem_a_output: true,
    problem_a_output: Some("53974"),
    problem_b: b,
    problem_b_output: Some("52840"),
    print_problem_b_output: true,
};
fn a(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.chars().filter(|c| c.is_numeric());
            let first = iter.next().map(|c| c.to_digit(10).unwrap()).unwrap();

            let last = iter
                .last()
                .map(|c| c.to_digit(10))
                .unwrap_or(Some(first))
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}
fn handle_line(input: &str) -> u32 {
    let substrings = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    let mut index_vec = substrings
        .iter()
        .flat_map(|(s_str, s_num)| {
            input
                .match_indices(s_str)
                .map(|(index, _str)| (index, *s_num))
        })
        .collect::<Vec<_>>();
    index_vec.sort_by(|a, b| a.0.cmp(&b.0));
    let first = index_vec.first().unwrap().1;
    let last = index_vec.last().unwrap().1;

    first * 10 + last
}
fn b(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| handle_line(line))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "142")
    }
    #[test]
    fn test_b() {
        let r = b("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
");
        assert_eq!(&r, "281");
    }
}
