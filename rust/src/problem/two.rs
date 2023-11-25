use super::Problem;
pub const TWO: Problem = Problem {
    number: 2,
    problem_a: a,
    problem_b: b,
};
#[derive(Copy, Clone, Debug)]
enum PlayResult {
    Win,
    Loose,
    Draw,
}
impl PlayResult {
    fn get_score(&self) -> u32 {
        match self {
            PlayResult::Win => 6,
            PlayResult::Loose => 0,
            PlayResult::Draw => 3,
        }
    }
    fn from_player_str(input: &str) -> Self {
        match input {
            "X" => Self::Loose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("invalid str: {}", input),
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct Round {
    opponent: Play,
    player: Play,
}
impl Round {
    fn get_score(&self) -> u32 {
        let won = match self.player {
            Play::Rock => match self.opponent {
                Play::Rock => PlayResult::Draw,
                Play::Paper => PlayResult::Loose,
                Play::Scissors => PlayResult::Win,
            },
            Play::Paper => match self.opponent {
                Play::Rock => PlayResult::Win,
                Play::Paper => PlayResult::Draw,
                Play::Scissors => PlayResult::Loose,
            },
            Play::Scissors => match self.opponent {
                Play::Rock => PlayResult::Loose,
                Play::Paper => PlayResult::Win,
                Play::Scissors => PlayResult::Draw,
            },
        };
        won.get_score() + self.player.get_score()
    }
}
#[derive(Copy, Clone, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}
impl Play {
    fn from_player_str(input: &str) -> Self {
        match input {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!("invalid input: {}", input),
        }
    }
    fn from_opponent_str(input: &str) -> Self {
        match input {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("invalid input: {}", input),
        }
    }
    fn get_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn get_player_move(&self, desired_result: PlayResult) -> Self {
        match desired_result {
            PlayResult::Win => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            PlayResult::Draw => match self {
                Self::Rock => Self::Rock,
                Self::Paper => Self::Paper,
                Self::Scissors => Self::Scissors,
            },
            PlayResult::Loose => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }
}
fn a(input: &str) -> String {
    let total_score: u32 = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut lines = line.split_whitespace();
            if let Some(opponent) = lines.next() {
                let opponent = Play::from_opponent_str(opponent);
                if let Some(player) = lines.next() {
                    let player = Play::from_player_str(player);
                    Some(Round { opponent, player })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|round| round.get_score())
        .sum();

    total_score.to_string()
}
fn b(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut split = line.split_whitespace();
            if let Some(opponent) = split.next() {
                let opponent_play = Play::from_opponent_str(opponent);
                if let Some(player) = split.next() {
                    Some((opponent_play, PlayResult::from_player_str(player)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(opponent_move, desired_result)| {
            Round {
                opponent: opponent_move,
                player: opponent_move.get_player_move(desired_result),
            }
            .get_score()
        })
        .sum::<u32>()
        .to_string()
}
#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
A Y
B X
C Z
    "#;
    #[test]
    fn test_a() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "15")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "12");
    }
}
