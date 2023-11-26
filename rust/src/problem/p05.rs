use super::Problem;
use std::collections::BTreeMap;

pub const P_05: Problem = Problem {
    number: 5,
    problem_a: a,
    problem_a_output: Some("FWSHSPJWM"),
    problem_b: b,
    problem_b_output: Some("PWPWHGFZS"),
};
#[derive(Clone, Debug)]
struct Stack {
    contents: BTreeMap<u32, Vec<char>>,
}
impl Stack {
    fn from_str(input: &str) -> Self {
        let mut contents: BTreeMap<u32, Vec<char>> = BTreeMap::new();
        for line in input.lines() {
            for (index, val) in line.chars().enumerate().filter(|(_idx, char)| {
                !(char.is_whitespace() || *char == '[' || *char == ']') && char.is_alphabetic()
            }) {
                let bucket_index = (index as u32 - 1) / 4;
                if let Some(bucket) = contents.get_mut(&bucket_index) {
                    bucket.push(val);
                } else {
                    contents.insert(bucket_index, vec![val]);
                }
            }
        }
        for (_key, bucket) in contents.iter_mut() {
            bucket.reverse();
        }
        Self { contents }
    }
    fn apply(&mut self, command: &MoveCommand) {
        for _ in 0..command.number {
            let take = self
                .contents
                .get_mut(&command.source)
                .unwrap()
                .pop()
                .unwrap();
            self.contents
                .get_mut(&command.destination)
                .unwrap()
                .push(take);
        }
    }
    fn apply_multiple(&mut self, command: &MoveCommand) {
        let mut take = (0..command.number)
            .map(|_| {
                self.contents
                    .get_mut(&command.source)
                    .unwrap()
                    .pop()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        take.reverse();
        let place = self.contents.get_mut(&command.destination).unwrap();
        for char in take {
            place.push(char)
        }
    }
    fn end_message(&self) -> String {
        self.contents
            .iter()
            .map(|(_key, bucket)| bucket.last())
            .map(|char| match char {
                Some(c) => *c,
                None => ' ',
            })
            .fold(String::new(), |acc, x| acc + &x.to_string())
    }
}
#[derive(Clone, Debug)]
struct MoveCommand {
    number: u32,
    source: u32,
    destination: u32,
}
impl MoveCommand {
    fn from_str(input: &str) -> Option<Self> {
        let mut split = input.split_whitespace();
        if split.next().is_none() {
            return None;
        }
        let number = if let Some(quantity) = split.next() {
            quantity.parse::<u32>().expect("failed to parse")
        } else {
            return None;
        };
        if split.next().is_none() {
            return None;
        }
        let source = if let Some(quantity) = split.next() {
            quantity.parse::<u32>().expect("failed to parse") - 1
        } else {
            return None;
        };
        if split.next().is_none() {
            return None;
        }
        let destination = if let Some(quantity) = split.next() {
            quantity.parse::<u32>().expect("failed to parse") - 1
        } else {
            return None;
        };
        Some(Self {
            number,
            source,
            destination,
        })
    }
}
fn a(input: &str) -> String {
    let mut split_contents = input.split("\n\n").filter(|s| !s.is_empty());
    let start_stack_str = split_contents.next().unwrap();
    let instructions_str = split_contents.next().unwrap();

    let mut stack = Stack::from_str(start_stack_str);

    instructions_str
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| MoveCommand::from_str(line))
        .for_each(|command| stack.apply(&command));

    stack.end_message()
}
fn b(input: &str) -> String {
    let mut split_contents = input.split("\n\n").filter(|s| !s.is_empty());
    let start_stack_str = split_contents.next().unwrap();
    let instructions_str = split_contents.next().unwrap();

    let mut stack = Stack::from_str(start_stack_str);

    instructions_str
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| MoveCommand::from_str(line))
        .for_each(|command| stack.apply_multiple(&command));
    stack.end_message()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "CMZ")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "MCD");
    }
}
