/*
    Run all state graph algorithms on every example input.

    The list of known examples is unfortunately maintained in several places:
    there is the actual folder examples/ containing everything, a list in
    example_gen of which are generated automatically by that binary, a
    partial list in test_examples of only those examples used as unit tests,
    and finally a list here of those examples included in run_all.
*/

use state_graph::driver;
use structopt::StructOpt;

const ALL_EXAMPLES: &[&str] = &[
    "1",
    "2",
    "3",
    "line_3",
    "line_10",
    "line_20",
    "line_100",
    "line_1000",
    "line_10000",
    "liveline_3",
    "liveline_10",
    "liveline_20",
    "liveline_100",
    "liveline_1000",
    "liveline_10000",
    "liveloop_3",
    "liveloop_10",
    "liveloop_20",
    "liveloop_100",
    "liveloop_1000",
    "liveloop_10000",
    "loop_3",
    "loop_10",
    "loop_20",
    "loop_100",
    "loop_1000",
    "loop_10000",
    "reverseline_3",
    "reverseline_10",
    "reverseline_20",
    "reverseline_100",
    "reverseline_1000",
    "reverseline_10000",
    "reverseliveline_3",
    "reverseliveline_10",
    "reverseliveline_20",
    "reverseliveline_100",
    "reverseliveline_1000",
    "reverseliveline_10000",
    "reverseliveloop_3",
    "reverseliveloop_10",
    "reverseliveloop_20",
    "reverseliveloop_100",
    "reverseliveloop_1000",
    "reverseliveloop_10000",
    "reverseloop_3",
    "reverseloop_10",
    "reverseloop_20",
    "reverseloop_100",
    "reverseloop_1000",
    "reverseloop_10000",
    "tree_3",
];

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run all",
    about = "Run all algorithms on every known example input."
)]
struct Args {
    #[structopt(short, long, default_value = "10")]
    timeout: u64,
}
impl Args {
    fn run(&self) {
        let mut results = Vec::new();
        for prefix in ALL_EXAMPLES {
            let result = driver::run_compare(prefix, self.timeout);
            results.push(result);
        }
        println!("========== RESULTS ==========");
        println!("{:?}", results);
    }
}

fn main() {
    Args::from_args().run();
}
