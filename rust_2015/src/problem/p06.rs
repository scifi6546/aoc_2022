use super::Problem;
pub struct P06;
impl Problem for P06 {
    fn number() -> u32 {
        6
    }

    type AOutput = usize;
    type BOutput = u32;

    fn a(input: &str) -> Option<Self::AOutput> {
        const X_SIZE:usize = 1000;
        const Y_SIZE:usize = 1000;
        fn get_coord(x:usize,y:usize)-> usize{
            y*X_SIZE + x
        }
        fn process_line(line: &str,grid: &mut Vec<bool>){
            let line_vals = line.split_whitespace().collect::<Vec<_>>();
            println!("line vals: {:?}",line_vals);
            todo!()
        }
        let mut grid = vec![false;X_SIZE*Y_SIZE];
        for line in input.lines(){
            process_line(line,&mut grid);
        }
        Some(grid.iter().filter(|is_on| **is_on).count())

    }

    fn b(input: &str) -> Option<Self::BOutput> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(P06::a("turn on 0,0 through 999,999").unwrap(), 1000 * 1000);
        assert_eq!(
            P06::a("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0").unwrap(),
            999 * 1000
        );
        assert_eq!(P06::a("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500").unwrap(),999*1000 -4 );
    }
}
