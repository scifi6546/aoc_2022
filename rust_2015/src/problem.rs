use std::{fs::File,io::Read};


mod p01;
mod p02;
mod p03;

pub trait Problem {
    /// problem number
    fn number() -> u32;

    type AOutput: std::fmt::Display;
    type BOutput: std::fmt::Display;
    fn a(input: &str) -> Self::AOutput;
    fn b(input: &str) -> Self::BOutput;
}
trait ProblemConcrete {
    fn number(&self) -> u32;
    fn a(&self, input: &str) -> String;
    fn b(&self, input: &str) -> String;
}
struct ProblemStruct<T: Problem> {
    _problem: T,
}
impl<T: Problem> ProblemConcrete for ProblemStruct<T> {
    fn number(&self) -> u32 {
        T::number()
    }
    fn a(&self, input: &str) -> String {
        T::a(input).to_string()
    }
    fn b(&self, input: &str) -> String {
        T::b(input).to_string()
    }
}
fn problem_list() -> Vec<Box<dyn ProblemConcrete>> {
    vec![Box::new(ProblemStruct { _problem: p01::P01 {} }),Box::new(ProblemStruct { _problem: p02::P02 {} }),Box::new(ProblemStruct { _problem: p03::P03 {} })]
}
pub struct ProblemRunner {
    problems: Vec<Box<dyn ProblemConcrete>>,
}
impl ProblemRunner {
    pub fn new() -> Self {
        let mut problems = problem_list();
        problems.sort_by(|a, b| a.number().cmp(&b.number()));
        Self { problems }
    }
    pub fn run(&self) {
        for problem in self.problems.iter(){
            println!("******* {}",problem.number());
            let mut file = File::open(format!("./input/{}.txt",problem.number())).expect("failed to open input for problem");
            let mut data_string = String::new();
            file.read_to_string(&mut data_string).expect("failed to read string");
            println!("a: {}",problem.a(&data_string));
            println!("b: {}",problem.b(&data_string));
        }
    }
}
