use super::Problem;
use std::cmp::max;
pub const ONE: Problem = Problem {
    number: 0,
    problem_a: a,
    problem_b: b,
};
fn a(input: &str) -> String {
    let elves = input
        .split("\n\n")
        .map(|elf| {
            elf.split_whitespace()
                .map(|food| food.parse::<u32>().expect("failed to parse"))
                .sum::<u32>()
        })
        .fold(0u32, |acc, x| max(acc, x));
    format!("{}", elves)
}
fn b(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one() {
        let input = r#"
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
        let r = a(input);
        assert_eq!(&r, "24000")
    }
}
