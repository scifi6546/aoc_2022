use crate::problem::Problem;
use std::collections::{BTreeMap, BTreeSet};

pub const P_04: Problem = Problem {
    number: 4,
    problem_a: a,
    print_problem_a_output: true,
    problem_a_output: Some("17803"),
    problem_b: b,
    problem_b_output: Some("5554894"),
    print_problem_b_output: true,
};
#[derive(Clone, Debug)]
struct Table {
    cards: Vec<Card>,
}
impl Table {
    fn from_str(input: &str) -> Self {
        let cards = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| Card::from_str(line))
            .collect();

        Self { cards }
    }
    fn get_score(&self) -> u32 {
        self.cards.iter().map(|card| card.get_score()).sum()
    }
    fn run_cards(&self) -> u32 {
        let mut number_of_cards = self.cards.iter().map(|_c| 1u32).collect::<Vec<_>>();
        for (i, card) in self.cards.iter().enumerate() {
            let card_num = card.get_matching();
            for j in (i + 1..i + card_num as usize + 1) {
                let add_num = number_of_cards[i];
                if j < number_of_cards.len() {
                    number_of_cards[j] += add_num;
                }
            }
        }
        number_of_cards.iter().sum()
    }
}
#[derive(Clone, Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    current_numbers: Vec<u32>,
    card_number: u32,
}
impl Card {
    fn from_str(input: &str) -> Self {
        let mut halves = input.split('|');

        let mut iter = halves.next().unwrap().split_whitespace().skip(1);
        let card_number = iter
            .next()
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()
            .unwrap();
        let winning_numbers = iter.map(|num| num.parse().unwrap()).collect();
        Self {
            winning_numbers,
            card_number,
            current_numbers: halves
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num_str| num_str.parse().unwrap())
                .collect(),
        }
    }
    fn get_matching(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|num| self.current_numbers.contains(*num))
            .count() as u32
    }
    fn get_score(&self) -> u32 {
        let matching_num = self.get_matching();
        if matching_num != 0 {
            2_u32.pow(matching_num - 1)
        } else {
            0
        }
    }
}
fn a(input: &str) -> String {
    let table = Table::from_str(input);

    table.get_score().to_string()
}

fn b(input: &str) -> String {
    let table = Table::from_str(input);

    table.run_cards().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    #[test]
    fn test_a() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "13")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "30");
    }
}
