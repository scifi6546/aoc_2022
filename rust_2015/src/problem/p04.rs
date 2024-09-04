use super::Problem;
use rayon::prelude::*;
pub struct P04;
impl Problem for P04 {
    fn number() -> u32 {
        4
    }

    type AOutput = u32;
    type BOutput = u32;

    fn a(input: &str) -> Option<Self::AOutput> {
        fn is_zeros(digest: &[u8; 16]) -> bool {
            digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0
        }
        Some(compute_value(input, is_zeros))
    }

    fn b(input: &str) -> Option<Self::BOutput> {
        fn is_zeros(digest: &[u8; 16]) -> bool {
            digest[0] == 0 && digest[1] == 0 && digest[2] == 0
        }
        Some(compute_value(input, is_zeros))
    }
}
fn compute_value<F: Fn(&[u8; 16]) -> bool>(input: &str, compare_function: F) -> u32 {
    let thread_size = 100;
    for index in 0..(u32::MAX) {
        let compute_val = format!("{}{}", input, index);
        let digest = md5::compute(compute_val);
        if compare_function(&digest) {
            return index;
        }
    }
    panic!("not found")
}
mod test {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(P04::a("abcdef").unwrap(), 609043);
        assert_eq!(P04::a("pqrstuv").unwrap(), 1048970);
    }
}
