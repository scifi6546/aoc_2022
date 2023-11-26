use super::Problem;

pub const P_06: Problem = Problem {
    number: 6,
    problem_a: a,
    problem_a_output: Some("1287"),
    problem_b: b,
    problem_b_output: Some("3716"),
};
#[derive(Clone, Debug)]
struct CharBuffer {
    chars: [Option<char>; 3],
}
impl CharBuffer {
    fn new() -> Self {
        Self { chars: [None; 3] }
    }
    fn push(&mut self, c: char) {
        if self.chars[0].is_none() {
            self.chars[0] = Some(c)
        } else if self.chars[1].is_none() {
            self.chars[1] = Some(c)
        } else if self.chars[2].is_none() {
            self.chars[2] = Some(c)
        } else {
            self.chars[0] = self.chars[1];
            self.chars[1] = self.chars[2];
            self.chars[2] = Some(c);
        };
    }
    fn is_unique(&self, c: char) -> bool {
        if self.chars[0].is_none() || self.chars[1].is_none() || self.chars[2].is_none() {
            false
        } else {
            let c = Some(c);
            !(self.chars[0] == self.chars[1]
                || self.chars[0] == self.chars[2]
                || self.chars[1] == self.chars[2]
                || c == self.chars[0]
                || c == self.chars[1]
                || c == self.chars[2])
        }
    }
}
fn a(input: &str) -> String {
    let mut buffer = CharBuffer::new();
    for (index, char) in input.chars().filter(|c| c.is_alphabetic()).enumerate() {
        if buffer.is_unique(char) {
            return (index + 1).to_string();
        }

        buffer.push(char);
    }
    String::new()
}
fn b(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "7");
        assert_eq!(a("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(a("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "");
    }
}
