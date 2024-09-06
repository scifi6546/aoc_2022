use super::Problem;
pub struct P09;
impl Problem for P09 {
    fn number() -> u32 {
        8
    }

    type AOutput = u32;
    type BOutput = u32;

    fn a(_input: &str) -> Option<Self::AOutput> {
        None
    }

    fn b(_input: &str) -> Option<Self::BOutput> {
        None
    }
}