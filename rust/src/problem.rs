use std::{fs::File, io::prelude::*, path::PathBuf};

mod p01;

mod p04;
mod p05;
mod p09;
mod p11;
mod p14;
mod p15;
mod p19;

mod p03;
mod p06;
mod p07;
mod p08;
mod p10;
mod p12;
mod p13;

mod p02;
mod p16;
mod p17;
mod p18;
mod p20;
mod p21;
mod p22;
mod p23;
mod p24;
mod p25;

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
                p01::P_01,
                p02::P_02,
                p03::P_03,
                p04::P_04,
                p05::P_05,
                p06::P_06,
                p07::P_07,
                p08::P_08,
                p09::P_09,
                p10::P_10,
                p11::P_11,
                p12::P_12,
                p13::P_13,
                p14::P_14,
                p15::P_15,
                p16::P_16,
                p17::P_17,
                p18::P_18,
                p19::P_19,
                p20::P_20,
                p21::P_21,
                p22::P_22,
                p23::P_23,
                p24::P_24,
                p25::P_25,
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
