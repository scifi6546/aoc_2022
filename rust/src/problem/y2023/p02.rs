use crate::problem::Problem;
use std::cmp::max;

pub const P_02: Problem = Problem {
    number: 2,
    problem_a: a,
    print_problem_a_output: true,
    problem_a_output: Some("2162"),
    problem_b: b,
    problem_b_output: Some("72513"),
    print_problem_b_output: true,
};
#[derive(Clone, Debug)]
struct Game {
    number: u32,
    rounds: Vec<Round>,
}
impl Game {
    pub fn from_str(input: &str) -> Self {
        let halves = input
            .split(":")
            .take(2)
            .map(|part| part.trim())
            .collect::<Vec<_>>();

        let number = halves[0]
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let rounds = halves[1]
            .split(";")
            .map(|part| part.trim())
            .map(|part| Round::from_str(part))
            .collect();

        Self { number, rounds }
    }
    fn is_valid(&self) -> bool {
        self.rounds
            .iter()
            .map(|round| round.is_valid())
            .fold(true, |acc, x| acc && x)
    }
    fn get_power(&self) -> u32 {
        self.rounds
            .iter()
            .map(|round| (round.red, round.green, round.blue))
            .fold([0, 0, 0], |acc, (x_r, x_g, x_b)| {
                [max(acc[0], x_r), max(acc[1], x_g), max(acc[2], x_b)]
            })
            .iter()
            .fold(1, |acc, x| acc * x)
    }
}
#[derive(Clone, Copy, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}
impl Round {
    fn from_str(input: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        input.split(",").map(|part| part.trim()).for_each(|part| {
            let number_pair = part.split_whitespace().collect::<Vec<_>>();
            let number = number_pair[0].parse::<u32>().unwrap();
            match number_pair[1] {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                _ => panic!(" invalid color: {}", number_pair[1]),
            }
        });
        Self { red, green, blue }
    }
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}
fn a(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Game::from_str(line))
        .filter(|game| game.is_valid())
        .map(|game| game.number)
        .sum::<u32>()
        .to_string()
}

fn b(input: &str) -> String {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Game::from_str(line).get_power())
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "8")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "2286");
    }
}
