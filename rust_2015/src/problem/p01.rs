use super::Problem;
pub struct P01;
impl Problem for P01 {
    fn number() -> u32 {
        1
    }

    type AOutput = i32;
    type BOutput = i32;

    fn a(input: &str) -> Self::AOutput {
        input
            .chars()
            .map(|c| match c {
                '(' => 1i32,
                ')' => -1,
                _ => panic!("unsupported symbol: {}", c),
            })
            .sum()
    }

    fn b(input: &str) -> Self::BOutput {
        let size_iter = input.chars().map(|c| match c {
            '(' => 1i32,
            ')' => -1,
            _ => panic!("unsupported symbol: {}", c),
        });
        let mut current_level = 0i32;
        for (idx, direction) in size_iter.enumerate() {
            current_level += direction;
            if current_level == -1 {
                return idx as i32 + 1;
            }
        }
        panic!("never hit basement")
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn a(){
        assert_eq!(P01::a("(())"), 0);
        assert_eq!(P01::a("()()"), 0);
        assert_eq!(P01::a("((("), 3);
        assert_eq!(P01::a("(()(()("), 3);
        assert_eq!(P01::a("))((((("), 3);
        assert_eq!(P01::a("())"), -1);
        assert_eq!(P01::a("))("), -1);

        assert_eq!(P01::a(")))"), -3);
        assert_eq!(P01::a(")())())"), -3);
    }
    #[test]
    fn b(){
        assert_eq!(P01::b(")"), 1);
        assert_eq!(P01::b("()())"), 5);
    }
}