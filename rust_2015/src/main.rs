mod problem;
use clap::Parser;

use problem::ProblemRunner;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "*")]
    problems: String,
}
fn main() {
    let args = Args::parse();
    let problem = ProblemRunner::builder().run_problems(args.problems).build();

    problem.run();
}
