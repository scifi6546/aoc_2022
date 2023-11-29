use super::Problem;
use std::collections::BTreeSet;
pub const P_06: Problem = Problem {
    number: 6,
    problem_a: a,
    problem_a_output: Some("1287"),
    print_problem_a_output: true,
    problem_b: b,
    problem_b_output: Some("3716"),
    print_problem_b_output: true,
};
#[derive(Clone, Debug)]
struct CharBuffer {
    chars: Vec<char>,
    buffer_length: usize,
}
impl CharBuffer {
    fn new(buffer_length: usize) -> Self {
        Self {
            chars: Vec::new(),
            buffer_length,
        }
    }
    fn push(&mut self, c: char) {
        if self.chars.len() == self.buffer_length {
            for i in 1..self.chars.len() {
                self.chars[i - 1] = self.chars[i];
            }
            self.chars[self.buffer_length - 1] = c;
        } else {
            self.chars.push(c);
        }
    }
    fn is_unique(&self) -> bool {
        if self.chars.iter().cloned().collect::<BTreeSet<_>>().len() == self.buffer_length {
            true
        } else {
            false
        }
    }
}
fn a(input: &str) -> String {
    let mut buffer = CharBuffer::new(4);
    for (index, char) in input.chars().filter(|c| c.is_alphabetic()).enumerate() {
        buffer.push(char);
        if buffer.is_unique() {
            return (index + 1).to_string();
        }
    }
    String::new()
}
fn b(input: &str) -> String {
    let mut buffer = CharBuffer::new(14);
    for (index, char) in input.chars().filter(|c| c.is_alphabetic()).enumerate() {
        buffer.push(char);
        if buffer.is_unique() {
            return (index + 1).to_string();
        }
    }
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
        assert_eq!(&r, "19");
        assert_eq!(b("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(b("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
