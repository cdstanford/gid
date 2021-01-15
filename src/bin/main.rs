/*
    Basic CLI
*/

use state_graph::driver::{self, Algorithm};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run basic example",
    about = "Run a state graph algorithm on an example input."
)]
struct Args1 {
    ex_name: String,

    #[structopt(short, long, default_value = "Naive")]
    algorithm: Algorithm,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run perf/stats comparison",
    about = "Run all state graph algorithm on an example input, compare stats."
)]
struct Args2 {
    ex_name: String,
}

#[derive(StructOpt)]
#[structopt(name = "state graph command line")]
enum SubComs {
    RunExample(Args1),
    StatsComparison(Args2),
}

fn main() {
    match SubComs::from_args() {
        SubComs::RunExample(args1) => {
            driver::run_example(&args1.ex_name, args1.algorithm);
        }
        SubComs::StatsComparison(args2) => {
            driver::run_compare(&args2.ex_name);
        }
    }
}
