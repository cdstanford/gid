/*
    Simplest binary: run a state graph algorithm on an example input
*/

use state_graph::constants::EX_TOPDIR;
use state_graph::driver::{self, Algorithm};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run basic example",
    about = "Run a state graph algorithm on an example input."
)]
struct Args {
    // e.g. "examples/handwritten"
    dir: String,
    // e.g. "2"
    ex_name: String,

    #[structopt(short, long, default_value = "Naive")]
    algorithm: Algorithm,

    #[structopt(short, long, default_value = "10")]
    timeout: u64,
}
impl Args {
    fn run(self) {
        let dir_path = format!("{}/{}", EX_TOPDIR, self.dir);
        driver::run_single_example(
            &dir_path,
            &self.ex_name,
            self.algorithm,
            self.timeout,
        );
    }
}

fn main() {
    Args::from_args().run();
}
