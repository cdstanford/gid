/*
    Binary to run all state graph algorithms on an example input
    and compare stats.
*/

use state_graph::driver::{self, Algorithm};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run perf/stats comparison",
    about = "Run all state graph algorithms on an example input, compare stats."
)]
struct Args {
    #[structopt(help = "Path to example without file extension \
                        (e.g. 'examples/handwritten/2')")]
    basename: String,

    #[structopt(short, long, help = "List of algorithms to exclude")]
    exclude: Vec<Algorithm>,

    #[structopt(short, long, default_value = "10")]
    timeout: u64,
}
impl Args {
    fn run(&self) {
        let algs = driver::algs_excluding(&self.exclude);
        driver::run_compare(&self.basename, &algs, self.timeout);
    }
}

fn main() {
    Args::from_args().run();
}
