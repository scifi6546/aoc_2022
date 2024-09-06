use super::Problem;
use std::{
    collections::{HashMap, HashSet},
    convert::From,
};

pub struct P07 {}
impl Problem for P07 {
    fn number() -> u32 {
        7
    }

    type AOutput = u16;
    type BOutput = u16;

    fn a(input: &str) -> Option<Self::AOutput> {
        let state = input
            .lines()
            .map(|line| parse_instruction(line))
            .fold(HashMap::new(), |registers, instruction| {
                process_instruction(instruction, registers)
            });

        let solved = solve_state(state);
        Some(solved["a"])
    }

    fn b(input: &str) -> Option<Self::BOutput> {
        let mut state = input
            .lines()
            .map(|line| parse_instruction(line))
            .fold(HashMap::new(), |registers, instruction| {
                process_instruction(instruction, registers)
            });

        let solved_a_value = solve_state(state.clone())["a"];
        state.insert(
            "b".to_string(),
            StatementValue::Constant(RegisterValue::Constant(solved_a_value)),
        );
        Some(solve_state(state.clone())["a"])
    }
}
type RegisterName = String;
type StateContainer = HashMap<RegisterName, StatementValue>;
type SolvedState = HashMap<RegisterName, u16>;
/// Represents the value in a statement (IE the left side)
#[derive(Clone, Debug)]
enum StatementValue {
    Constant(RegisterValue),
    And {
        source_a: RegisterValue,
        source_b: RegisterValue,
    },
    Or {
        source_a: RegisterValue,
        source_b: RegisterValue,
    },
    LeftShift {
        /// the source
        source: RegisterValue,
        /// the amount to shift by so if this value is 1 and source = a "a<<1" will be output
        amount: RegisterValue,
    },
    RightShift {
        /// the source
        source: RegisterValue,
        /// the amount to shift by so if this value is 1 and source = a "a<<1" will be output
        amount: RegisterValue,
    },
    Not(RegisterValue),
}
#[derive(Clone, Debug)]
enum RegisterValue {
    Constant(u16),
    Register(String),
}
#[derive(Clone, Debug)]
struct Instruction {
    destination: RegisterName,
    operation: Operation,
}
#[derive(Clone, Debug)]
enum Operation {
    Constant {
        value: RegisterValue,
    },
    And {
        source_a: RegisterValue,
        source_b: RegisterValue,
    },
    Or {
        source_a: RegisterValue,
        source_b: RegisterValue,
    },
    LeftShift {
        /// the source
        source: RegisterValue,
        /// the amount to shift by so if this value is 1 and source = a "a<<1" will be output
        amount: RegisterValue,
    },
    RightShift {
        /// the source
        source: RegisterValue,
        /// the amount to shift by so if this value is 1 and source = a "a>>1" will be output
        amount: RegisterValue,
    },
    Not(RegisterValue),
}
fn is_numeric(input: &str) -> bool {
    input
        .chars()
        .map(|c| c.is_numeric())
        .fold(true, |acc, x| acc && x)
}
impl RegisterValue {
    fn from_str(input: &str) -> Self {
        if is_numeric(input) {
            Self::Constant(input.parse().expect("symbol is not a number"))
        } else {
            Self::Register(input.to_string())
        }
    }
}
fn solve_state(mut state: StateContainer) -> SolvedState {
    /// if the variable is constant tries to get it
    fn try_constant(
        value: &StatementValue,
        overall_state: &StateContainer,
        recurse_level: u32,
    ) -> Option<u16> {
        match value {
            StatementValue::Constant(register_value) => {
                try_value(register_value, overall_state, recurse_level + 1)
            }
            StatementValue::Or { source_a, source_b } => {
                let source_a = try_value(source_a, overall_state, recurse_level + 1);
                let source_b = try_value(source_b, overall_state, recurse_level + 1);
                if source_a.is_some() && source_b.is_some() {
                    Some(source_a.unwrap() | source_b.unwrap())
                } else {
                    None
                }
            }
            StatementValue::Not(value) => {
                if let Some(value) = try_value(value, overall_state, recurse_level + 1) {
                    Some(!value)
                } else {
                    None
                }
            }
            StatementValue::And { source_a, source_b } => {
                let source_a = try_value(source_a, overall_state, recurse_level + 1);
                let source_b = try_value(source_b, overall_state, recurse_level + 1);
                if source_a.is_some() && source_b.is_some() {
                    Some(source_a.unwrap() & source_b.unwrap())
                } else {
                    None
                }
            }
            StatementValue::LeftShift { source, amount } => {
                let source = try_value(source, overall_state, recurse_level + 1);
                let amount = try_value(amount, overall_state, recurse_level + 1);
                if source.is_some() && amount.is_some() {
                    Some(source.unwrap() << amount.unwrap())
                } else {
                    None
                }
            }
            StatementValue::RightShift { source, amount } => {
                let source = try_value(source, overall_state, recurse_level + 1);
                let amount = try_value(amount, overall_state, recurse_level + 1);
                if source.is_some() && amount.is_some() {
                    Some(source.unwrap() >> amount.unwrap())
                } else {
                    None
                }
            }
        }
    }
    fn try_name(
        register: &RegisterName,
        state: &StateContainer,
        recurse_level: u32,
    ) -> Option<u16> {
        try_constant(&state[register], state, recurse_level + 1)
    }
    fn try_value(
        register_value: &RegisterValue,
        overall_state: &StateContainer,
        recurse_level: u32,
    ) -> Option<u16> {
        match register_value {
            RegisterValue::Register(name) => {
                if recurse_level < 20 {
                    try_name(name, overall_state, recurse_level + 1)
                } else {
                    None
                }
            }
            //RegisterValue::Register(name) => None,
            RegisterValue::Constant(v) => Some(*v),
        }
    }
    /// gets all the registers that depend on other registers
    fn get_variable_registers(state: &StateContainer) -> HashSet<RegisterName> {
        state
            .iter()
            .filter(|(name, value)| try_constant(value, state, 0).is_none())
            .map(|(name, _value)| name.to_string())
            .collect()
    }
    fn get_constant_registers(state: &StateContainer) -> HashMap<RegisterName, u16> {
        state
            .iter()
            .filter_map(|(name, value)| {
                if let Some(v) = try_constant(value, state, 0) {
                    Some((name.clone(), v))
                } else {
                    None
                }
            })
            .collect()
    }
    let mut constant_values = HashMap::new();
    let mut variable_values = state
        .iter()
        .map(|(name, _value)| name.clone())
        .collect::<HashSet<_>>();

    let mut first_run = true;
    let mut last_variable_len = variable_values.len();
    loop {
        if variable_values.len() == 0 {
            return constant_values;
        }

        let found_constants = variable_values
            .iter()
            .filter_map(|register| {
                if let Some(val) = try_name(register, &state, 0) {
                    Some((register.clone(), val))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();
        for (register, value) in found_constants {
            state.insert(
                register.clone(),
                StatementValue::Constant(RegisterValue::Constant(value)),
            );

            variable_values.remove(&register);
            constant_values.insert(register, value);
        }

        if !first_run {
            if last_variable_len == variable_values.len() {
                panic!("made no progress")
            }
        } else {
            first_run = false;
            last_variable_len = variable_values.len();
        }
    }
}
fn parse_instruction(line: &str) -> Instruction {
    let symbols = line.split_whitespace().collect::<Vec<_>>();
    assert!(symbols.len() >= 3);
    if symbols[1] == "->" {
        let value = RegisterValue::from_str(symbols[0]);
        let destination = symbols[2].to_string();

        Instruction {
            destination,
            operation: Operation::Constant { value },
        }
    } else if symbols[1] == "AND" {
        let source_a = RegisterValue::from_str(symbols[0]);
        let source_b = RegisterValue::from_str(symbols[2]);
        let destination = symbols[4].to_string();

        Instruction {
            destination,
            operation: Operation::And { source_a, source_b },
        }
    } else if symbols[1] == "OR" {
        let source_a = RegisterValue::from_str(symbols[0]);
        let source_b = RegisterValue::from_str(symbols[2]);
        let destination = symbols[4].to_string();
        Instruction {
            destination,
            operation: Operation::Or { source_a, source_b },
        }
    } else if symbols[1] == "LSHIFT" {
        let source = RegisterValue::from_str(symbols[0]);
        let amount = RegisterValue::from_str(symbols[2]);
        let destination = symbols[4].to_string();
        Instruction {
            destination,
            operation: Operation::LeftShift { source, amount },
        }
    } else if symbols[1] == "RSHIFT" {
        let source = RegisterValue::from_str(symbols[0]);
        let amount = RegisterValue::from_str(symbols[2]);
        let destination = symbols[4].to_string();
        Instruction {
            destination,
            operation: Operation::RightShift { source, amount },
        }
    } else if symbols[0] == "NOT" {
        let destination = symbols[3].to_string();
        let source = RegisterValue::from_str(symbols[1]);
        Instruction {
            destination,
            operation: Operation::Not(source),
        }
    } else {
        panic!("invalid operation: \"{}\"", line)
    }
}
fn process_instruction(instruction: Instruction, mut states: StateContainer) -> StateContainer {
    let dest = instruction.destination.clone();
    let value = match instruction.operation {
        Operation::Constant { value } => StatementValue::Constant(value),
        Operation::And { source_a, source_b } => StatementValue::And { source_a, source_b },
        Operation::Or { source_a, source_b } => StatementValue::Or { source_a, source_b },
        Operation::LeftShift { source, amount } => StatementValue::LeftShift { source, amount },
        Operation::RightShift { source, amount } => StatementValue::RightShift { source, amount },
        Operation::Not(value) => StatementValue::Not(value),
    };
    states.insert(dest, value);
    states
}
fn get_signal_output(input: &str) -> SolvedState {
    let state = input
        .lines()
        .map(|line| parse_instruction(line))
        .fold(HashMap::new(), |registers, instruction| {
            process_instruction(instruction, registers)
        });

    solve_state(state)
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a() {
        let mut expected_output = HashMap::new();
        expected_output.insert("d".to_string(), 72);
        expected_output.insert("e".to_string(), 507);
        expected_output.insert("f".to_string(), 492);
        expected_output.insert("g".to_string(), 114);
        expected_output.insert("h".to_string(), 65412);
        expected_output.insert("i".to_string(), 65079);
        expected_output.insert("x".to_string(), 123);
        expected_output.insert("y".to_string(), 456);
        assert_eq!(
            get_signal_output(
                "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
"
            ),
            expected_output
        );
    }
}
