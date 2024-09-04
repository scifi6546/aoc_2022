use super::Problem;
pub struct P01;
impl Problem for P01 {
    fn number() -> u32 {
        1
    }

    type AOutput = i32;
    type BOutput = i32;

    fn a(input: &str) -> Option<Self::AOutput> {
        Some(
            input
                .chars()
                .map(|c| match c {
                    '(' => 1i32,
                    ')' => -1,
                    _ => panic!("unsupported symbol: {}", c),
                })
                .sum(),
        )
    }

    fn b(input: &str) -> Option<Self::BOutput> {
        let size_iter = input.chars().map(|c| match c {
            '(' => 1i32,
            ')' => -1,
            _ => panic!("unsupported symbol: {}", c),
        });
        let mut current_level = 0i32;
        for (idx, direction) in size_iter.enumerate() {
            current_level += direction;
            if current_level == -1 {
                return Some(idx as i32 + 1);
            }
        }
        panic!("never hit basement")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(P01::a("(())").unwrap(), 0);
        assert_eq!(P01::a("()()").unwrap(), 0);
        assert_eq!(P01::a("(((").unwrap(), 3);
        assert_eq!(P01::a("(()(()(").unwrap(), 3);
        assert_eq!(P01::a("))(((((").unwrap(), 3);
        assert_eq!(P01::a("())").unwrap(), -1);
        assert_eq!(P01::a("))(").unwrap(), -1);

        assert_eq!(P01::a(")))").unwrap(), -3);
        assert_eq!(P01::a(")())())").unwrap(), -3);
    }
    #[test]
    fn b() {
        assert_eq!(P01::b(")").unwrap(), 1);
        assert_eq!(P01::b("()())").unwrap(), 5);
    }
}
