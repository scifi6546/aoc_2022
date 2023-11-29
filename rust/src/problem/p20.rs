use super::Problem;

pub const P_20: Problem = Problem {
    number: 20,
    problem_a: a,
    problem_a_output: None,
    print_problem_a_output: true,
    problem_b: b,
    problem_b_output: None,
    print_problem_b_output: true,
};

fn a(_input: &str) -> String {
    String::new()
}
fn b(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
    "#;
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
