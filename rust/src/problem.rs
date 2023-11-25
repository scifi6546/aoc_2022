use std::{fs::File, io::prelude::*, path::Path};
mod one;
mod two;

pub struct Problem {
    number: u32,
    problem_a: fn(&str) -> String,
    problem_b: fn(&str) -> String,
}
impl Problem {
    fn run_final(&self) -> std::io::Result<()> {
        let mut input_file = File::open(format!("./input/{}.txt", self.number))?;
        let mut input_data = String::new();
        input_file.read_to_string(&mut input_data)?;
        let problem_a_answer = (self.problem_a)(&input_data);
        let problem_b_answer = (self.problem_b)(&input_data);
        println!("problem a: {}", problem_a_answer);
        println!("problem b: {}", problem_b_answer);
        Ok(())
    }
}
trait SubProblem {
    fn run_str(input: &str) -> String;
    fn run_path<P: AsRef<Path>>(p: P) -> std::io::Result<String> {
        let mut file = File::open(p)?;
        let mut input_data = String::new();
        file.read_to_string(&mut input_data)?;
        Self::run_str(&input_data);
        Ok(Self::run_str(&input_data))
    }
}
pub struct ProblemRunner {
    problems: Vec<Problem>,
}
impl ProblemRunner {
    pub fn new() -> Self {
        Self {
            problems: vec![one::ONE, two::TWO],
        }
    }
    pub fn run(&self) -> std::io::Result<()> {
        for problem in self.problems.iter() {
            problem.run_final()?
        }
        Ok(())
    }
}
