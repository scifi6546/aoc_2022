use super::Problem;
use std::collections::{BTreeSet, HashMap};
pub struct P05;
impl Problem for P05 {
    fn number() -> u32 {
        5
    }

    type AOutput = usize;
    type BOutput = usize;

    fn a(input: &str) -> Option<Self::AOutput> {
        Some(input.lines().filter(|line| check_row(line)).count())
    }

    fn b(input: &str) -> Option<Self::BOutput> {
        Some(input.lines().filter(|line| check_row_b(line)).count())
    }
}
fn check_row(row: &str) -> bool {
    fn contains_three_vowels(input: &str) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        input.chars().filter(|c| vowels.contains(c)).count() >= 3
    }
    fn contains_double_letter(input: &str) -> bool {
        let mut iter = input.chars();
        let mut previous_letter = iter.next().unwrap();

        for letter in iter {
            if letter == previous_letter {
                return true;
            }

            previous_letter = letter;
        }
        return false;
    }
    fn contains_bad_string(input: &str) -> bool {
        let bad_strings = ["ab", "cd", "pq", "xy"];
        bad_strings
            .iter()
            .filter(|swear| input.contains(**swear))
            .count()
            >= 1
    }

    contains_three_vowels(row) && contains_double_letter(row) && !contains_bad_string(row)
}
fn check_row_b(line: &str) -> bool {
    fn contains_two_pairs(input: &str) -> bool {
        let mut char_iter = input.chars();
        if let Some(first) = char_iter.next() {
            let mut previous_char = first;
            let mut map = HashMap::<[char; 2], Vec<usize>>::new();
            for (idx, current_char) in char_iter.enumerate() {
                let start_index = idx;
                let pair = [previous_char, current_char];
                if let Some(index_vec) = map.get_mut(&pair) {
                    for index in index_vec.iter() {
                        if start_index - index >= 2 {
                            return true;
                        }
                    }
                    index_vec.push(start_index)
                } else {
                    map.insert(pair, vec![start_index]);
                }
                previous_char = current_char;
            }

            false
        } else {
            false
        }
    }
    fn contains_repeat(input: &str) -> bool {
        let mut map = HashMap::<char, usize>::new();
        for (idx, char) in input.chars().enumerate() {
            if let Some(last_idx) = map.get_mut(&char) {
                if idx - *last_idx == 2 {
                    return true;
                } else {
                    *last_idx = idx;
                }
            } else {
                map.insert(char, idx);
            }
        }
        false
    }
    contains_two_pairs(line) && contains_repeat(line)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(P05::a("ugknbfddgicrmopn").unwrap(), 1);
        assert_eq!(P05::a("aaa").unwrap(), 1);
        assert_eq!(P05::a("jchzalrnumimnmhp").unwrap(), 0);
        assert_eq!(P05::a("haegwjzuvuyypxyu").unwrap(), 0);
        assert_eq!(P05::a("dvszwmarrgswjxmb").unwrap(), 0);
    }
    #[test]
    fn a_two_row() {
        let str = "ugknbfddgicrmopn\nhaegwjzuvuyypxyu";
        assert_eq!(P05::a(str).unwrap(), 1);
    }
    #[test]
    fn b() {
        assert_eq!(P05::b("qjhvhtzxzqqjkmpb").unwrap(), 1);
        assert_eq!(P05::b("xxyxx").unwrap(), 1);
        assert_eq!(P05::b("uurcxstgmygtbstg").unwrap(), 0);
        assert_eq!(P05::b("ieodomkazucvgmuy").unwrap(), 0);
    }
}
