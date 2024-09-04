use super::Problem;
use std::collections::BTreeSet;
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}
pub struct P03;
impl Problem for P03 {
    fn number() -> u32 {
        3
    }

    type AOutput = usize;
    type BOutput = usize;

    fn a(input: &str) -> Option<Self::AOutput> {
        Some(santa_tracker::<1>(input))
    }

    fn b(input: &str) -> Option<Self::BOutput> {
        Some(santa_tracker::<2>(input))
    }
}
fn santa_tracker<const N: usize>(input: &str) -> usize {
    let mut santas = [Coord { x: 0, y: 0 }; N];
    let mut visited = BTreeSet::new();
    for santa_pos in santas.iter() {
        visited.insert(*santa_pos);
    }

    for (i, char) in input.chars().enumerate() {
        let current_pos = &mut santas[i % santas.len()];
        match char {
            '>' => current_pos.x += 1,
            '<' => current_pos.x -= 1,
            '^' => current_pos.y += 1,
            'v' => current_pos.y -= 1,
            _ => panic!("invalid char: {}", char),
        }
        visited.insert(*current_pos);
    }

    visited.len()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(P03::a(">").unwrap(), 2);
        assert_eq!(P03::a("^>v<").unwrap(), 4);
        assert_eq!(P03::a("^v^v^v^v^v").unwrap(), 2);
    }
    #[test]
    fn b() {
        assert_eq!(P03::b("^v").unwrap(), 3);
        assert_eq!(P03::b("^>v<").unwrap(), 3);
        assert_eq!(P03::b("^v^v^v^v^v").unwrap(), 11);
    }
}
