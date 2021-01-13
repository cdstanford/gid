/*
    Basic CLI
*/

use state_graph::driver::{self, Algorithm};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run basic example",
    about = "Run a state graph algorithm on an example input."
)]
struct Args1 {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Expected output, if any
    #[structopt(short, long, parse(from_os_str))]
    expect_output: Option<PathBuf>,

    #[structopt(short, long, default_value = "Naive")]
    algorithm: Algorithm,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run perf/stats comparison",
    about = "Run all state graph algorithm on an example input, compare stats."
)]
struct Args2 {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Expected output
    #[structopt(parse(from_os_str))]
    expect_output: PathBuf,
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
            driver::run_example(
                &args1.input,
                args1.expect_output.as_ref(),
                args1.algorithm,
            );
        }
        SubComs::StatsComparison(args2) => {
            driver::run_compare(&args2.input, &args2.expect_output);
        }
    }
}
