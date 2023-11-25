use super::Problem;

pub const SIXTEEN: Problem = Problem {
    number: 16,
    problem_a: a,
    problem_a_output: None,
    problem_b: b,
    problem_b_output: None,
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
