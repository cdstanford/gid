/*
    Run all state graph algorithms on every known example input.

    See constants::ALL_EXAMPLE_DIRS for the list of known examples.
*/

use guided_digraph::constants::{ALL_EXAMPLE_DIRS, RESULTS_DIR};
use guided_digraph::driver::{self, Algorithm};
use guided_digraph::util;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "run all",
    about = "Run all algorithms on every known example input."
)]
struct Args {
    #[structopt(short, long, help = "List of algorithms to exclude")]
    exclude: Vec<Algorithm>,

    #[structopt(short, long, help = "Print files, without processing")]
    print: bool,

    #[structopt(short, long, default_value = "10")]
    timeout: u64,
}
impl Args {
    fn run(&self) {
        println!("========= Run All =========");
        let datetime = util::current_datetime_str();
        let mode = if cfg!(debug_assertions) { "debug" } else { "release" };
        let algs = driver::algs_excluding(&self.exclude);
        println!("Current Datetime: {:?}", datetime);
        println!("Mode: {}", mode);
        println!("Timeout: {}s", self.timeout);
        println!("Algs: {:?} (excluding {:?})", algs, self.exclude);
        let mut result_lines = vec![driver::run_compare_csv_header(&algs)];
        for dir in ALL_EXAMPLE_DIRS {
            println!("======= directory: {} =======", dir);
            for basename in driver::example_basenames_in_dir(dir) {
                if self.print {
                    println!("{}", basename);
                } else {
                    result_lines.push(driver::run_compare(
                        &basename,
                        &algs,
                        self.timeout,
                    ));
                }
            }
        }
        if !self.print {
            println!("========= Results =========");
            let filepath = format!(
                "{}/{}_{}_t{}.csv",
                RESULTS_DIR, datetime, mode, self.timeout
            );
            util::lines_to_file(&filepath, result_lines);
            println!("Results saved to: {}", filepath);
        }
    }
}

fn main() {
    Args::from_args().run();
}
