use std::path::PathBuf;
use std::{fs::File, io::prelude::*, path::Path};

mod one;

mod eight;
mod five;
mod four;
mod nine;
mod seven;
mod six;
mod ten;
mod three;
mod two;

pub struct Problem {
    number: u32,
    problem_a: fn(&str) -> String,
    problem_a_output: Option<&'static str>,
    problem_b: fn(&str) -> String,
    problem_b_output: Option<&'static str>,
}
impl Problem {
    fn get_file_path(&self) -> PathBuf {
        PathBuf::from(format!("./input/{}.txt", self.number))
    }
    fn run_final(&self) -> std::io::Result<()> {
        let input_path = self.get_file_path();
        if input_path.exists() {
            let mut input_file = File::open(self.get_file_path())?;
            println!("*********** problem {}", self.number);
            let mut input_data = String::new();
            input_file.read_to_string(&mut input_data)?;
            let problem_a_answer = (self.problem_a)(&input_data);

            let problem_b_answer = (self.problem_b)(&input_data);
            if let Some(correct_a_answer) = self.problem_a_output {
                if problem_a_answer == correct_a_answer {
                    println!("problem a: {} (Correct!)", problem_a_answer);
                } else {
                    println!("problem a: {} (NOT CORRECT!)", problem_a_answer);
                }
            } else {
                println!("problem a: {} (N/A)", problem_a_answer);
            }
            if let Some(correct_b_answer) = self.problem_b_output {
                if problem_b_answer == correct_b_answer {
                    println!("problem b: {} (Correct!)", problem_b_answer);
                } else {
                    println!("problem b: {} (NOT CORRECT!)", problem_b_answer);
                }
            } else {
                println!("problem b: {} (N/A)", problem_a_answer);
            }
        } else {
            println!("*********** problem {}", self.number);
        }

        Ok(())
    }
}
pub struct ProblemRunner {
    problems: Vec<Problem>,
}
impl ProblemRunner {
    pub fn new() -> Self {
        Self {
            problems: vec![
                one::ONE,
                two::TWO,
                three::THREE,
                four::FOUR,
                five::FIVE,
                six::SIX,
                seven::SEVEN,
                eight::EIGHT,
                nine::NINE,
                ten::TEN,
            ],
        }
    }
    pub fn run(&self) -> std::io::Result<()> {
        for problem in self.problems.iter() {
            let result = problem.run_final();
            if result.is_err() {
                break;
            }
        }
        Ok(())
    }
}
