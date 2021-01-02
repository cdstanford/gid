/*
    Basic CLI
*/

use state_graph::driver;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "state graph interface",
    about = "Run a state graph algorithm on an input or output file."
)]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Expected output, if any
    #[structopt(short, long, parse(from_os_str))]
    expect_output: Option<PathBuf>,
}
// TODO: add output file: output: Option<PathBuf>,

fn main() {
    let args = Args::from_args();
    driver::run_example(&args.input, args.expect_output.as_ref());
}
