/*
    Simplest binary: run a state graph algorithm on an example input
*/

use state_graph::driver::{self, Algorithm};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run basic example",
    about = "Run a state graph algorithm on an example input."
)]
struct Args {
    ex_name: String,

    #[structopt(short, long, default_value = "Naive")]
    algorithm: Algorithm,
}
impl Args {
    fn run(self) {
        driver::run_example(&self.ex_name, self.algorithm);
    }
}

fn main() {
    Args::from_args().run();
}
