use std::cmp::min;
use super::Problem;
pub struct P02{}
impl Problem for P02{
    fn number() -> u32 {
        2
    }

    type AOutput = i32;
    type BOutput = i32;

    fn a(input: &str) -> Self::AOutput {
        input.split_whitespace().map(|line| a_row(line)).sum()
    }

    fn b(input: &str) -> Self::BOutput {
        input.split_whitespace().map(|line| b_row(line)).sum()
    }
}
fn b_row(row: &str) -> i32 {
    let sides = row
        .split("x")
        .map(|side| side.parse::<u32>().expect("failed to make sides"))
        .collect::<Vec<_>>();
    let l = sides[0];
    let w = sides[1];
    let h = sides[2];

    let min_perimeter = min(min(2 * l + 2 * w, 2 * l + 2 * h), 2 * w + 2 * h) as i32;
    let volume = l * w * h;
    min_perimeter + volume as i32
}
fn a_row(row: &str) -> i32 {
    let sides = row
        .split("x")
        .map(|side| side.parse::<u32>().expect("failed to make sides"))
        .collect::<Vec<_>>();
    let l = sides[0];
    let w = sides[1];
    let h = sides[2];
    let total = 2 * l * w + 2 * w * h + 2 * h * l;
    let min_side = min(min(l * w, w * h), h * l);
    (total + min_side) as i32
}
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn a(){
        assert_eq!(P02::a("2x3x4"), 58);
        assert_eq!(P02::a("1x1x10"), 43);
    }
    #[test]
    fn b(){
        assert_eq!(P02::b("2x3x4"), 34);
        assert_eq!(P02::b("1x1x10"), 14);
    }
}