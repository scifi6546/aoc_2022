use super::Problem;
use std::{cmp::Reverse, collections::BinaryHeap};
pub const P_12: Problem = Problem {
    number: 12,
    problem_a: a,
    problem_a_output: Some("361"),
    print_problem_a_output: true,
    problem_b: b,
    problem_b_output: Some("354"),
    print_problem_b_output: true,
};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Height(u8),
    Start,
    End,
}

impl Tile {
    fn can_get_to(&self, tile: Tile) -> bool {
        match self {
            Tile::Height(h) => match tile {
                Tile::Height(h_n) => h + 1 >= h_n,
                Tile::Start => true,
                Tile::End => *h >= 24,
            },
            Tile::Start => match tile {
                Tile::Height(h) => {
                    if h == 0 {
                        true
                    } else {
                        false
                    }
                }
                Tile::Start => true,
                Tile::End => false,
            },
            Tile::End => panic!("already at end"),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TileCost {
    distance: u32,
    location: (isize, isize),
}
impl std::cmp::Ord for TileCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .cmp(&other.distance)
            .then_with(|| self.location.0.cmp(&other.location.0))
            .then_with(|| self.location.1.cmp(&other.location.1))
    }
}
impl std::cmp::PartialOrd for TileCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Tile {
    fn from_char(c: char) -> Self {
        if c.is_ascii_lowercase() {
            let mut code = [0u8];
            c.encode_utf8(&mut code);

            Self::Height(code[0] - 0x61)
        } else {
            match c {
                'S' => Self::Start,
                'E' => Self::End,
                _ => panic!("Unsupported char: {}", c),
            }
        }
    }
}
fn get_shortest(tiles: Vec<Vec<Tile>>, start_x: isize, start_y: isize) -> Option<u32> {
    let mut distance = tiles
        .iter()
        .map(|row| {
            row.iter()
                .map(|tile| if *tile == Tile::Start { 0 } else { u32::MAX })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(TileCost {
        distance: 0,
        location: (start_x, start_y),
    }));
    loop {
        let lowest_cost = heap.pop().unwrap().0;

        let (lowest_x, lowest_y) = lowest_cost.location;
        let neighbor_tiles = [
            (lowest_x - 1, lowest_y),
            (lowest_x + 1, lowest_y),
            (lowest_x, lowest_y - 1),
            (lowest_x, lowest_y + 1),
        ];
        let lowest_tile = tiles[lowest_x as usize][lowest_y as usize];
        if lowest_tile == Tile::End {
            return Some(lowest_cost.distance);
        }
        let neighbor_tiles = neighbor_tiles
            .iter()
            .filter(|(x, y)| *x >= 0 && *x < tiles.len() as isize)
            .filter(|(x, y)| *y >= 0 && *y < tiles[*x as usize].len() as isize)
            .filter(|(x, y)| lowest_tile.can_get_to(tiles[*x as usize][*y as usize]))
            .filter(|(x, y)| distance[*x as usize][*y as usize] > lowest_cost.distance + 1)
            .copied()
            .collect::<Vec<_>>();

        neighbor_tiles.iter().cloned().for_each(|(tile_x, tile_y)| {
            heap.push(Reverse(TileCost {
                distance: lowest_cost.distance + 1,
                location: (tile_x, tile_y),
            }));
            distance[tile_x as usize][tile_y as usize] = lowest_cost.distance + 1;
        });
    }
}
fn a(input: &str) -> String {
    let tiles = input
        .lines()
        .map(|line| line.trim())
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().map(|c| Tile::from_char(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_x, start_y) = tiles
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(y, tile)| **tile == Tile::Start)
                .map(|(y, _tile)| (x as isize, y as isize))
                .collect::<Vec<_>>()
        })
        .filter(|row| !row.is_empty())
        .flatten()
        .next()
        .unwrap();
    let mut distance = tiles
        .iter()
        .map(|row| {
            row.iter()
                .map(|tile| if *tile == Tile::Start { 0 } else { u32::MAX })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(TileCost {
        distance: 0,
        location: (start_x, start_y),
    }));
    loop {
        let lowest_cost = heap.pop().unwrap().0;

        let (lowest_x, lowest_y) = lowest_cost.location;
        let neighbor_tiles = [
            (lowest_x - 1, lowest_y),
            (lowest_x + 1, lowest_y),
            (lowest_x, lowest_y - 1),
            (lowest_x, lowest_y + 1),
        ];
        let lowest_tile = tiles[lowest_x as usize][lowest_y as usize];
        if lowest_tile == Tile::End {
            return lowest_cost.distance.to_string();
        }
        let neighbor_tiles = neighbor_tiles
            .iter()
            .filter(|(x, y)| *x >= 0 && *x < tiles.len() as isize)
            .filter(|(x, y)| *y >= 0 && *y < tiles[*x as usize].len() as isize)
            .filter(|(x, y)| lowest_tile.can_get_to(tiles[*x as usize][*y as usize]))
            .filter(|(x, y)| distance[*x as usize][*y as usize] > lowest_cost.distance + 1)
            .copied()
            .collect::<Vec<_>>();

        neighbor_tiles.iter().cloned().for_each(|(tile_x, tile_y)| {
            heap.push(Reverse(TileCost {
                distance: lowest_cost.distance + 1,
                location: (tile_x, tile_y),
            }));
            distance[tile_x as usize][tile_y as usize] = lowest_cost.distance + 1;
        });
    }
}
fn b(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "31")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "29");
    }
}
