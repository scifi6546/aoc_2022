use crate::problem::Problem;
use std::cmp::max;

pub const P_01: Problem = Problem {
    number: 1,
    problem_a: a,
    print_problem_a_output: true,
    problem_a_output: None,
    problem_b: b,
    problem_b_output: None,
    print_problem_b_output: true,
};
fn a(input: &str) -> String {
    String::new()
}
fn b(input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "";
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "");
    }
}
