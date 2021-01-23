/*
    Binary to run all state graph algorithms on an example input
    and compare stats.
*/

use state_graph::driver;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run perf/stats comparison",
    about = "Run all state graph algorithms on an example input, compare stats."
)]
struct Args {
    ex_name: String,

    #[structopt(short, long, default_value = "10")]
    timeout: u64,
}
impl Args {
    fn run(&self) {
        driver::run_compare(&self.ex_name, self.timeout);
    }
}

fn main() {
    Args::from_args().run();
}
