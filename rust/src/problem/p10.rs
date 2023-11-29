use super::Problem;

pub const P_10: Problem = Problem {
    number: 10,
    problem_a: a,
    problem_a_output: Some("12520"),
    problem_b: b,
    problem_b_output: None,
};
#[derive(Clone, Copy, Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}
#[derive(Clone, Copy, Debug)]
struct CpuState {
    cycle_count: u32,
    register_x: i32,
}
impl CpuState {
    fn new() -> Self {
        Self {
            cycle_count: 1,
            register_x: 1,
        }
    }
    fn run_instruction(&self, instruction: Instruction) -> Vec<Self> {
        match instruction {
            Instruction::NoOp => vec![Self {
                cycle_count: self.cycle_count + 1,
                register_x: self.register_x,
            }],
            Instruction::AddX(amount) => vec![
                Self {
                    cycle_count: self.cycle_count + 1,
                    register_x: self.register_x,
                },
                Self {
                    cycle_count: self.cycle_count + 2,
                    register_x: self.register_x + amount,
                },
            ],
        }
    }
}
impl Instruction {
    fn from_str(input: &str) -> Option<Self> {
        let input_vec = input.split_whitespace().take(2).collect::<Vec<_>>();
        if input_vec.len() >= 1 {
            match input_vec[0] {
                "noop" => Some(Instruction::NoOp),
                "addx" => Some(Instruction::AddX(
                    input_vec[1].parse().expect("failed to parse"),
                )),
                _ => panic!("unsupported instruction {:#?}", input_vec[0]),
            }
        } else {
            None
        }
    }
}
fn a(input: &str) -> String {
    let instructions = input
        .lines()
        .filter_map(|line| Instruction::from_str(line))
        .collect::<Vec<_>>();

    let mut state = CpuState::new();
    let mut signal_strength_sum = 0;
    for ins in instructions.iter() {
        let out_state = state.run_instruction(*ins);
        for state in out_state.iter() {
            if (state.cycle_count as i32 - 20) % 40 == 0 {
                signal_strength_sum += state.cycle_count as i32 * state.register_x
            }
        }
        state = *out_state.last().unwrap();
    }
    signal_strength_sum.to_string()
}
fn b(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "13140")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "");
    }
}
