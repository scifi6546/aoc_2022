use crate::problem::Problem;
use std::collections::HashMap;
const START_STR: &'static str = "seed";
const END_STR: &'static str = "location";
pub const P_05: Problem = Problem {
    number: 5,
    problem_a: a,
    print_problem_a_output: true,
    problem_a_output: None,
    problem_b: b,
    problem_b_output: None,
    print_problem_b_output: true,
};
type Seed = u64;
#[derive(Clone, Copy, Debug)]
struct Range {
    destination_start: Seed,
    source_start: Seed,
    range_length: Seed,
}

impl Range {
    fn from_str(input: &str) -> Option<Self> {
        let numbers = input
            .split_whitespace()
            .filter_map(|num_str| num_str.parse::<Seed>().ok())
            .collect::<Vec<_>>();
        if numbers.len() == 3 {
            Some(Self {
                destination_start: numbers[0],
                source_start: numbers[1],
                range_length: numbers[2],
            })
        } else {
            None
        }
    }
    fn to_location(&self, seed_number: Seed) -> Option<Seed> {
        if seed_number >= self.source_start && seed_number <= self.source_start + self.range_length
        {
            Some(seed_number - self.source_start + self.destination_start)
        } else {
            None
        }
    }
    fn map_range(&self, seed_range: SeedRange) -> SeedRangeResult {
        let mut in_range = Vec::new();
        let mut out_of_range_seeds = Vec::new();
        let seeds = (seed_range.start..seed_range.start + seed_range.number_seeds);
        for seed in seeds {
            if let Some(mapped_seed) = self.to_location(seed) {
                in_range.push(mapped_seed)
            } else {
                out_of_range_seeds.push(seed);
            }
        }
        let in_range = if !in_range.is_empty() {
            Some(SeedRange {
                start: in_range[0],
                number_seeds: in_range.len() as Seed,
            })
        } else {
            None
        };
        let mut out_of_range = Vec::new();
        let mut last_seed: Option<Seed> = None;
        let mut last_start: Option<Seed> = None;
        for seed in out_of_range_seeds {
            if last_start.is_none() {
                last_start = Some(seed);
            } else {
                if let Some(last_seed) = last_seed {
                    if seed - last_seed > 1 {
                        out_of_range.push(SeedRange {
                            start: last_start.unwrap(),
                            number_seeds: last_seed - last_start.unwrap() + 1,
                        });
                        last_start = Some(seed);
                    }
                }
            }

            last_seed = Some(seed);
        }
        SeedRangeResult {
            in_range,
            out_of_range,
        }
    }
}
#[derive(Clone, Debug)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}
impl Map {
    fn from_str(input: &str) -> Self {
        let mut lines_iter = input.split("\n");
        let start_str = lines_iter
            .next()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .split("-to-")
            .map(|s| s.trim())
            .collect::<Vec<_>>();
        let ranges = lines_iter
            .filter_map(|range_str| Range::from_str(range_str))
            .collect::<Vec<_>>();
        Self {
            source: start_str[0].to_string(),
            destination: start_str[1].to_string(),
            ranges,
        }
    }
    fn to_location(&self, seed: Seed) -> Seed {
        self.ranges
            .iter()
            .filter_map(|range| range.to_location(seed))
            .next()
            .unwrap_or(seed)
    }
    fn map_range(&self, seed_range: SeedRange) -> Vec<SeedRange> {
        println!("self ranges: {:#?}", self.ranges);
        let mut mapped_ranges = Vec::new();
        let mut unmapped_ranges = vec![seed_range];
        for range in self.ranges.iter() {
            if let Some(un_mapped) = unmapped_ranges.pop() {
                let mut map_result = range.map_range(un_mapped);
                if let Some(mapped) = map_result.in_range {
                    mapped_ranges.push(mapped)
                }
                unmapped_ranges.append(&mut map_result.out_of_range);
            } else {
                return mapped_ranges;
            }
        }
        mapped_ranges.append(&mut unmapped_ranges);
        println!("mapped range: {:#?}", mapped_ranges);
        return mapped_ranges;
    }
}

struct SeedRangeResult {
    in_range: Option<SeedRange>,
    out_of_range: Vec<SeedRange>,
}
#[derive(Clone, Debug)]
struct MapCollection {
    seeds: Vec<Seed>,
    maps: HashMap<String, Map>,
}
impl MapCollection {
    const START_STR: &'static str = "seed";
    const END_STR: &'static str = "location";
    pub fn from_str(input: &str, seed_parser: fn(&str) -> Vec<Seed>) -> Self {
        let mut chunks = input.split("\n\n");
        let seeds = seed_parser(chunks.next().unwrap());
        Self {
            seeds,
            maps: chunks
                .map(|chunk| Map::from_str(chunk))
                .map(|map| (map.source.clone(), map))
                .collect::<HashMap<_, _>>(),
        }
    }
    fn get_seed_location_recursive(&self, seed: Seed, current_str: &str) -> Seed {
        if current_str != Self::END_STR {
            let current_map = &self.maps[current_str];
            self.get_seed_location_recursive(
                current_map.to_location(seed),
                &current_map.destination,
            )
        } else {
            return seed;
        }
    }
    fn get_seed_location(&self, seed: Seed) -> Seed {
        self.get_seed_location_recursive(seed, Self::START_STR)
    }
    pub fn get_min_seed_location(&self) -> Seed {
        self.seeds
            .iter()
            .map(|seed| self.get_seed_location(*seed))
            .min()
            .unwrap()
    }
    pub fn get_min_in_range(&self, seed_range: SeedRange) -> Seed {
        self.map_range_location(seed_range)
            .iter()
            .inspect(|r| println!("range: {:#?}", r))
            .map(|range| range.start)
            .min()
            .unwrap()
    }
    fn map_range_location(&self, seed_range: SeedRange) -> Vec<SeedRange> {
        self.map_range_location_recurse(seed_range, Self::START_STR)
    }
    fn map_range_location_recurse(
        &self,
        seed_range: SeedRange,
        current_str: &str,
    ) -> Vec<SeedRange> {
        if current_str != Self::END_STR {
            let map = &self.maps[current_str];

            map.map_range(seed_range)
                .drain(..)
                .inspect(|r| println!("map range: {:#?}", r))
                .flat_map(|range| self.map_range_location_recurse(range, &map.destination))
                .collect()
        } else {
            vec![seed_range]
        }
    }
}
#[derive(Clone, Copy, Debug)]
struct SeedRange {
    start: Seed,
    number_seeds: Seed,
}
impl SeedRange {
    fn from_str(input: &str) -> Vec<Self> {
        println!("input: {}", input);
        let numbers = input
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse::<Seed>().ok())
            .collect::<Vec<_>>();
        println!("numbers: {:#?}", numbers);
        (0..numbers.len() / 2)
            .map(|i| i * 2)
            .map(|i| SeedRange {
                start: numbers[i],
                number_seeds: numbers[i + 1],
            })
            .collect()
    }
}
fn a(input: &str) -> String {
    fn parse_seeds(input: &str) -> Vec<Seed> {
        input
            .split_whitespace()
            .skip(1)
            .filter_map(|seed_str| seed_str.parse::<Seed>().ok())
            .collect::<Vec<_>>()
    }
    let collection = MapCollection::from_str(input, parse_seeds);
    collection.get_min_seed_location().to_string()
}

fn b(input: &str) -> String {
    fn parse_seeds(input: &str) -> Vec<Seed> {
        let values = input.split_whitespace().skip(1).collect::<Vec<_>>();
        (0..values.len() / 2)
            .flat_map(|i| {
                let [range_start, range_length] =
                    [i * 2, i * 2 + 1].map(|i| values[i].parse::<Seed>().unwrap());

                (range_start..(range_start + range_length)).collect::<Vec<_>>()
            })
            .collect()
    }
    let seed_ranges = SeedRange::from_str(input.split("\n\n").next().unwrap());
    let collection = MapCollection::from_str(input, parse_seeds);
    seed_ranges
        .iter()
        .map(|range| collection.get_min_in_range(*range))
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    #[test]
    fn test_a() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "35")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "46");
    }
}
