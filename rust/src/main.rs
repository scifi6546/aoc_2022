mod problem;

fn main() {
    let problems = problem::ProblemRunner::new();
    problems.run().expect("failed to run");
}
