/*
    Run all state graph algorithms on every known example input.

    See constants::ALL_EXAMPLES for the list of known examples.
*/

use state_graph::constants::{ALL_EXAMPLES, RESULTS_DIR};
use state_graph::driver;
use state_graph::util;
use structopt::StructOpt;

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
        println!("======= Run All =======");
        let datetime = util::current_datetime_str();
        let mode = if cfg!(debug_assertions) { "debug" } else { "release" };
        println!("Current Datetime: {:?}", datetime);
        println!("Mode: {}", mode);
        println!("Timeout: {}s", self.timeout);
        let mut result_lines = Vec::new();
        result_lines.push(driver::run_compare_csv_header());
        for (dir, example) in ALL_EXAMPLES {
            let basename = format!("{}/{}", dir, example);
            let result = driver::run_compare(&basename, self.timeout);
            result_lines.push(result);
        }
        println!("======= Results =======");
        let filepath = format!(
            "{}/{}_{}_t{}.csv",
            RESULTS_DIR, datetime, mode, self.timeout
        );
        util::lines_to_file(&filepath, result_lines);
        println!("Results saved to: {}", filepath);
    }
}

fn main() {
    Args::from_args().run();
}
