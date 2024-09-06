use super::Problem;
pub struct P06;
#[derive(Copy, Clone, Debug)]
struct IntCoord {
    pub x: usize,
    pub y: usize,
}
#[derive(Copy, Clone, Debug)]
enum Instruction {
    TurnOn { start: IntCoord, end: IntCoord },
    TurnOff { start: IntCoord, end: IntCoord },
    Toggle { start: IntCoord, end: IntCoord },
}
impl IntCoord {
    pub fn from_str(input: &str) -> Self {
        let mut coords = input.split(",").take(2);
        Self {
            x: coords
                .next()
                .expect("failed to get x")
                .parse()
                .expect("failed to parse coords"),
            y: coords
                .next()
                .expect("failed to get x")
                .parse()
                .expect("failed to parse coords"),
        }
    }
}
impl Problem for P06 {
    fn number() -> u32 {
        6
    }

    type AOutput = usize;
    type BOutput = u32;

    fn a(input: &str) -> Option<Self::AOutput> {
        fn process_instruction(instruction: Instruction, mut grid: Vec<bool>) -> Vec<bool> {
            match instruction {
                Instruction::TurnOn { start, end } => {
                    for x in (start.x)..=(end.x) {
                        for y in (start.y)..=(end.y) {
                            grid[get_coord(x, y)] = true;
                        }
                    }
                    grid
                }
                Instruction::TurnOff { start, end } => {
                    for x in (start.x)..=(end.x) {
                        for y in (start.y)..=(end.y) {
                            grid[get_coord(x, y)] = false;
                        }
                    }
                    grid
                }
                Instruction::Toggle { start, end } => {
                    for x in (start.x)..=(end.x) {
                        for y in (start.y)..=(end.y) {
                            grid[get_coord(x, y)] = !grid[crate::problem::p06::get_coord(x, y)];
                        }
                    }
                    grid
                }
            }
        }

        let grid = vec![false; X_SIZE * Y_SIZE];

        Some(
            input
                .lines()
                .map(|line| parse_line(line))
                .fold(grid, |grid, instruction| {
                    process_instruction(instruction, grid)
                })
                .iter()
                .filter(|v| **v)
                .count(),
        )
    }

    fn b(input: &str) -> Option<Self::BOutput> {
        fn process_instruction(instruction: Instruction, mut grid: Vec<u32>) -> Vec<u32> {
            match instruction {
                Instruction::TurnOn { start, end } => {
                    for x in (start.x)..=(end.x) {
                        for y in (start.y)..=(end.y) {
                            grid[get_coord(x, y)] += 1;
                        }
                    }
                    grid
                }
                Instruction::TurnOff { start, end } => {
                    for x in (start.x)..=(end.x) {
                        for y in (start.y)..=(end.y) {
                            let old_val = grid[get_coord(x, y)];
                            if old_val >= 1 {
                                grid[get_coord(x, y)] = old_val - 1;
                            } else {
                                grid[get_coord(x, y)] = 0;
                            }
                        }
                    }
                    grid
                }
                Instruction::Toggle { start, end } => {
                    for x in (start.x)..=(end.x) {
                        for y in (start.y)..=(end.y) {
                            grid[get_coord(x, y)] += 2;
                        }
                    }
                    grid
                }
            }
        }
        let grid = vec![0u32; X_SIZE * Y_SIZE];

        Some(
            input
                .lines()
                .map(|line| parse_line(line))
                .fold(grid, |grid, instruction| {
                    process_instruction(instruction, grid)
                })
                .iter()
                .sum(),
        )
    }
}
const X_SIZE: usize = 1000;
const Y_SIZE: usize = 1000;
fn get_coord(x: usize, y: usize) -> usize {
    y * X_SIZE + x
}
fn parse_line(line: &str) -> Instruction {
    let line_words = line.split_whitespace().collect::<Vec<_>>();
    match line_words[0] {
        "turn" => {
            let start = IntCoord::from_str(line_words[2]);
            let end = IntCoord::from_str(line_words[4]);
            match line_words[1] {
                "on" => Instruction::TurnOn { start, end },
                "off" => Instruction::TurnOff { start, end },
                _ => panic!("invalid second word: {}", line_words[1]),
            }
        }
        "toggle" => {
            let start = IntCoord::from_str(line_words[1]);
            let end = IntCoord::from_str(line_words[3]);
            Instruction::Toggle { start, end }
        }
        _ => panic!(
            "invalid first word: {} full word: \"{}\"",
            line_words[0], line
        ),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(P06::a("turn on 0,0 through 999,999").unwrap(), 1000 * 1000);
    }
    #[test]
    fn a2() {
        assert_eq!(
            P06::a("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0").unwrap(),
            999 * 1000
        );
    }
    #[test]
    fn a3() {
        assert_eq!(P06::a("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500").unwrap(),999*1000 -4 );
    }
    #[test]
    fn b() {
        assert_eq!(P06::b("turn on 0,0 through 0,0").unwrap(), 1);
        assert_eq!(P06::b("toggle 0,0 through 999,999").unwrap(), 2000000)
    }
    #[test]
    fn b_on_off() {
        assert_eq!(
            P06::b("turn on 0,0 through 0,0\nturn off 0,0 through 0,0").unwrap(),
            0
        );
    }
}
