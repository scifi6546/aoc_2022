use super::Problem;
use std::collections::BTreeSet;
pub struct P03;
impl Problem for P03{
    fn number() -> u32 {
        3
    }

    type AOutput = usize;
    type BOutput = u32;

    fn a(input: &str) -> Self::AOutput {
        #[derive(Ord, PartialOrd, Eq, PartialEq,Clone,Copy)]
        struct Coord{
            x: i32,
            y:i32
        }
        let mut current_pos = Coord{x:0,y:0};
        let mut visited = BTreeSet::new();
        visited.insert(current_pos);
        for char in input.chars(){
            match char{
                '>' => { current_pos.x+=1 },
                '<' => {current_pos.x-=1},
                '^' => {current_pos.y+=1},
                'v' => {current_pos.y-=1},
                _ => panic!("invalid char: {}",char)
            }
            visited.insert(current_pos);
        }

        visited.len()
    }

    fn b(input: &str) -> Self::BOutput {
        todo!()
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn a(){
        assert_eq!(P03::a(">"),2);
        assert_eq!(P03::a("^>v<"),4);
        assert_eq!(P03::a("^v^v^v^v^v"),2);
    }
}