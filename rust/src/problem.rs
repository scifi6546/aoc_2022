use std::{fs::File, io::prelude::*, path::PathBuf};

mod y2022;
mod y2023;
pub struct Problem {
    number: u32,
    problem_a: fn(&str) -> String,
    problem_a_output: Option<&'static str>,
    print_problem_a_output: bool,
    problem_b: fn(&str) -> String,
    problem_b_output: Option<&'static str>,
    print_problem_b_output: bool,
}
impl Problem {
    fn get_file_path(&self, year: u32) -> PathBuf {
        PathBuf::from(format!("./input/{}/{}.txt", year, self.number))
    }
    fn run_final(&self, year: u32) -> std::io::Result<()> {
        let input_path = self.get_file_path(year);
        if input_path.exists() {
            let mut input_file = File::open(self.get_file_path(year))?;
            println!("*********** problem {}", self.number);
            let mut input_data = String::new();
            input_file.read_to_string(&mut input_data)?;
            let problem_a_answer = (self.problem_a)(&input_data);

            let problem_b_answer = (self.problem_b)(&input_data);
            if let Some(correct_a_answer) = self.problem_a_output {
                if self.print_problem_a_output {
                    if problem_a_answer == correct_a_answer {
                        println!("problem a: {} (Correct!)", problem_a_answer);
                    } else {
                        println!("problem a: {} (NOT CORRECT!)", problem_a_answer);
                    }
                } else {
                    if problem_a_answer == correct_a_answer {
                        println!("problem a: (Correct!)");
                    } else {
                        println!("problem a: (NOT CORRECT!)");
                    }
                }
            } else {
                println!("problem a: {} (N/A)", problem_a_answer);
            }
            if let Some(correct_b_answer) = self.problem_b_output {
                if self.print_problem_b_output {
                    if problem_b_answer == correct_b_answer {
                        println!("problem b: {} (Correct!)", problem_b_answer);
                    } else {
                        println!("problem b: {} (NOT CORRECT!)", problem_b_answer);
                    }
                } else {
                    if problem_b_answer == correct_b_answer {
                        println!("problem b: (Correct!)");
                    } else {
                        println!("problem b: (NOT CORRECT!)");
                    }
                }
            } else {
                println!("problem b: {} (N/A)", problem_b_answer);
            }
        } else {
            println!("*********** problem {}", self.number);
        }

        Ok(())
    }
}
struct Year {
    problems: Vec<Problem>,
    year: u32,
}
pub struct ProblemRunner {
    years: Vec<Year>,
}
impl ProblemRunner {
    pub fn new() -> Self {
        Self {
            years: vec![
                Year {
                    problems: vec![
                        y2022::p01::P_01,
                        y2022::p02::P_02,
                        y2022::p03::P_03,
                        y2022::p04::P_04,
                        y2022::p05::P_05,
                        y2022::p06::P_06,
                        y2022::p07::P_07,
                        y2022::p08::P_08,
                        y2022::p09::P_09,
                        y2022::p10::P_10,
                        y2022::p11::P_11,
                        y2022::p12::P_12,
                        y2022::p13::P_13,
                        y2022::p14::P_14,
                        y2022::p15::P_15,
                        y2022::p16::P_16,
                        y2022::p17::P_17,
                        y2022::p18::P_18,
                        y2022::p19::P_19,
                        y2022::p20::P_20,
                        y2022::p21::P_21,
                        y2022::p22::P_22,
                        y2022::p23::P_23,
                        y2022::p24::P_24,
                        y2022::p25::P_25,
                    ],
                    year: 2022,
                },
                Year {
                    problems: vec![
                        y2023::p01::P_01,
                        y2023::p02::P_02,
                        y2023::p03::P_03,
                        y2023::p04::P_04,
                    ],
                    year: 2023,
                },
            ],
        }
    }
    pub fn run(&self) -> std::io::Result<()> {
        for year in self.years.iter() {
            println!("******** {}", year.year);
            for problem in year.problems.iter() {
                let result = problem.run_final(year.year);
                if result.is_err() {
                    break;
                }
            }
        }

        Ok(())
    }
}
