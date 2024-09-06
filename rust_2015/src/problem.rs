use std::path::Path;
use std::{collections::BTreeSet, default::Default, fs::File, io::Read, time::Instant};

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
        Box::new(ProblemStruct {
            _problem: p07::P07 {},
        }),
        Box::new(ProblemStruct {
            _problem: p08::P08 {},
        }),
        Box::new(ProblemStruct {
            _problem: p09::P09 {},
        }),
    ]
}
#[derive(Clone, Debug)]
pub struct ProblemRunnerBuilder {
    run_problems: String,
}
impl ProblemRunnerBuilder {
    pub fn build(self) -> ProblemRunner {
        ProblemRunner::build(self)
    }
    pub fn run_problems(mut self, run_problems: String) -> Self {
        self.run_problems = run_problems;
        self
    }
}
impl Default for ProblemRunnerBuilder {
    fn default() -> Self {
        Self {
            run_problems: "*".to_string(),
        }
    }
}
pub struct ProblemRunner {
    run_list: BTreeSet<u32>,
    problems: Vec<Box<dyn ProblemConcrete>>,
}
impl ProblemRunner {
    pub fn builder() -> ProblemRunnerBuilder {
        ProblemRunnerBuilder::default()
    }
    fn build(builder: ProblemRunnerBuilder) -> Self {
        let mut problems = problem_list();
        problems.sort_by(|a, b| a.number().cmp(&b.number()));
        let run_list = if builder.run_problems == "*" {
            problems.iter().map(|t| t.number()).collect()
        } else {
            let problem: u32 = builder
                .run_problems
                .parse()
                .expect("failed to parse problem");
            let mut set = BTreeSet::new();
            set.insert(problem);
            set
        };
        Self { problems, run_list }
    }
    pub fn run(&self) {
        for problem in self.problems.iter() {
            if !self.run_list.contains(&problem.number()) {
                continue;
            }
            println!("******* {}", problem.number());
            let file_path = Path::new("./input").join(format!("{}.txt", problem.number()));
            if !file_path.exists() {
                println!("file does not exist skipping {}", problem.number());
                continue;
            }
            let mut file = File::open(file_path).expect("failed to open input for problem");
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
