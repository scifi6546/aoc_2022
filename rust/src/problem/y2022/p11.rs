use super::Problem;

pub const P_11: Problem = Problem {
    number: 11,
    problem_a: a,
    problem_a_output: Some("55930"),
    print_problem_a_output: true,
    problem_b: b,
    problem_b_output: None,
    print_problem_b_output: true,
};
#[derive(Clone, Copy, Debug)]
enum Operation {
    Multiply(Value, Value),
    Add(Value, Value),
}
impl Operation {
    fn from_str(input: &str) -> Self {
        let operation_line = input.split_whitespace().skip(3).collect::<Vec<_>>();
        let left_val = match operation_line[0] {
            "old" => Value::Old,
            _ => Value::Constant(operation_line[0].parse::<ItemType>().unwrap()),
        };
        let right_val = match operation_line[2] {
            "old" => Value::Old,
            _ => Value::Constant(operation_line[2].parse::<ItemType>().unwrap()),
        };
        match operation_line[1] {
            "*" => Self::Multiply(left_val, right_val),
            "+" => Self::Add(left_val, right_val),
            _ => panic!("invalid operation: {}", operation_line[1]),
        }
    }
    fn apply(&self, item: ItemType) -> ItemType {
        match self {
            Self::Add(val1, val2) => val1.get(item) + val2.get(item),
            Self::Multiply(val1, val2) => val1.get(item) * val2.get(item),
        }
    }
}
#[derive(Clone, Copy, Debug)]
enum Value {
    Old,
    Constant(u128),
}
impl Value {
    fn from_str(input: &str) -> Self {
        match input {
            "old" => Self::Old,
            _ => Self::Constant(input.parse::<ItemType>().unwrap()),
        }
    }
    fn get(&self, old_val: ItemType) -> ItemType {
        match self {
            Value::Old => old_val,
            Value::Constant(c) => *c,
        }
    }
}
#[derive(Clone, Copy, Debug)]
enum Test {
    DivisibleBy(ItemType),
}
impl Test {
    fn test_item(&self, item: ItemType) -> bool {
        match self {
            Test::DivisibleBy(c) => item % c == 0,
        }
    }
}
type ItemType = u128;
#[derive(Clone, Debug)]
struct Monkey {
    num_items_inspected: u64,
    items: Vec<ItemType>,
    operation: Operation,
    test: Test,
    true_monkey: usize,
    false_monkey: usize,
}
impl Monkey {
    fn from_str(input: &str) -> Option<Self> {
        let trimmed_input = input.strip_prefix("\n");
        let trimmed_input = match trimmed_input {
            Some(s) => s,
            None => input,
        };

        let lines = trimmed_input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        if lines.len() == 6 {
            let starting_items = lines[1]
                .split_whitespace()
                .skip(2)
                .map(|item| {
                    let suff = item.strip_suffix(",");
                    if let Some(num) = suff {
                        num
                    } else {
                        item
                    }
                })
                .map(|num| num.parse::<ItemType>().unwrap())
                .collect::<Vec<_>>();

            let operation = Operation::from_str(lines[2]);

            let test_line = lines[3].split_whitespace().skip(1).collect::<Vec<_>>();
            let test = match test_line[0] {
                "divisible" => Test::DivisibleBy(test_line[2].parse().unwrap()),
                _ => panic!("invalid test str: {}", test_line[0]),
            };

            let true_monkey = lines[4]
                .split_whitespace()
                .skip(5)
                .next()
                .unwrap()
                .parse()
                .unwrap();

            let false_monkey = lines[5]
                .split_whitespace()
                .skip(5)
                .next()
                .unwrap()
                .parse()
                .unwrap();

            Some(Self {
                num_items_inspected: 0,
                items: starting_items,
                operation,
                test,
                true_monkey,
                false_monkey,
            })
        } else {
            None
        }
    }
}
fn a(input: &str) -> String {
    let mut monkyes = input
        .split("\n\n")
        .filter_map(|string| Monkey::from_str(string))
        .collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkyes.len() {
            let monkey = &monkyes[i];
            let monkey_items = monkey
                .items
                .iter()
                .map(|item| {
                    let new_item = monkyes[i].operation.apply(*item) / 3;
                    if monkyes[i].test.test_item(new_item) {
                        (monkey.true_monkey as usize, new_item)
                    } else {
                        (monkey.false_monkey as usize, new_item)
                    }
                })
                .collect::<Vec<_>>();
            {
                let mut monkey = &mut monkyes[i];
                monkey.items.clear();
                monkey.num_items_inspected += monkey_items.len() as u64;
            }

            for (monkey_num, item) in monkey_items.iter() {
                monkyes[*monkey_num].items.push(*item)
            }
        }
    }
    let mut inspected_list = monkyes
        .iter()
        .map(|mon| mon.num_items_inspected)
        .collect::<Vec<_>>();
    inspected_list.sort();
    inspected_list
        .iter()
        .rev()
        .take(2)
        .fold(1, |acc, x| acc * x)
        .to_string()
}
fn b(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "10605")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "");
    }
}
