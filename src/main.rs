/*
    Entrypoint to run a series of transactions using the state graph
    interface, and collect the output.
*/

use state_graph::interface::{ExampleInput, ExampleOutput, StateGraph};
use state_graph::naive::NaiveStateGraph;
use state_graph::util::from_json_file;
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
    let example_string = ExampleInput(vec![
        state_graph::interface::Transaction::Add(1, 3),
        state_graph::interface::Transaction::Add(1, 2),
        state_graph::interface::Transaction::Done(2),
    ]);
    println!("example: {}", serde_json::to_string(&example_string).unwrap());

    let args = Args::from_args();
    let input: ExampleInput = from_json_file(&args.input);
    let expected_output: Option<ExampleOutput> =
        args.expect_output.as_ref().map(from_json_file);

    println!("===== State Graph =====");
    println!("Running naive algorithm...");
    let mut graph = NaiveStateGraph::new();
    graph.process_all(&input);
    let output = graph.collect_all();

    println!("=== Output ===");
    println!("{:?}", output);

    if let Some(expected) = expected_output {
        if output == expected {
            println!("Success: output is as expected");
        } else {
            println!("Failed: output is incorrect!");
            println!("=== Expected ===");
            println!("{:?}", expected);
        }
    }
}
