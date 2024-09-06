use std::{fs::File, io::Read, time::Instant};

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;

pub trait Problem {
    /// problem number
    fn number() -> u32;

    type AOutput: std::fmt::Display;
    type BOutput: std::fmt::Display;
    fn a(input: &str) -> Option<Self::AOutput>;
    fn b(input: &str) -> Option<Self::BOutput>;
}
trait ProblemConcrete {
    fn number(&self) -> u32;
    fn a(&self, input: &str) -> Option<String>;
    fn b(&self, input: &str) -> Option<String>;
}
struct ProblemStruct<T: Problem> {
    _problem: T,
}
impl<T: Problem> ProblemConcrete for ProblemStruct<T> {
    fn number(&self) -> u32 {
        T::number()
    }
    fn a(&self, input: &str) -> Option<String> {
        T::a(input).map(|v| v.to_string())
    }
    fn b(&self, input: &str) -> Option<String> {
        T::b(input).map(|v| v.to_string())
    }
}
fn problem_list() -> Vec<Box<dyn ProblemConcrete>> {
    vec![
        Box::new(ProblemStruct {
            _problem: p01::P01 {},
        }),
        Box::new(ProblemStruct {
            _problem: p02::P02 {},
        }),
        Box::new(ProblemStruct {
            _problem: p03::P03 {},
        }),
        Box::new(ProblemStruct {
            _problem: p04::P04 {},
        }),
        Box::new(ProblemStruct {
            _problem: p05::P05 {},
        }),
        Box::new(ProblemStruct {
            _problem: p06::P06 {},
        }),
        Box::new(ProblemStruct{
            _problem: p07::P07{}
        }
        )
    ]
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
        for problem in self.problems.iter() {
            println!("******* {}", problem.number());
            let mut file = File::open(format!("./input/{}.txt", problem.number()))
                .expect("failed to open input for problem");
            let mut data_string = String::new();
            file.read_to_string(&mut data_string)
                .expect("failed to read string");
            let start_time = Instant::now();
            if let Some(answer) = problem.a(&data_string) {
                let end_time = Instant::now();
                println!(
                    "a: {}, elapsed: {}s",
                    answer,
                    (end_time - start_time).as_millis() as f32 / 1000.0
                );
            } else {
                println!("a: Not Done")
            }
            let start_time = Instant::now();
            if let Some(answer) = problem.b(&data_string) {
                let end_time = Instant::now();
                println!(
                    "b: {}, elapsed: {}s",
                    answer,
                    (end_time - start_time).as_millis() as f32 / 1000.0
                );
            } else {
                println!("b: Not Done")
            }
        }
    }
}
