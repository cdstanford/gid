/*
    The driver:
    Handles running a series of transactions using the state
    graph interface, and collecting/viewing/checking the output.
*/

use super::algorithm::{
    JumpStateGraph, NaiveStateGraph, SimpleStateGraph, TarjanStateGraph,
};
use super::interface::{Example, ExampleResult, StateGraph};
use std::fmt::{self, Debug};
use std::str::FromStr;
use std::time::Duration;
use structopt::StructOpt;

/*
    Constants
*/
// Timeout to use when running asserted unit tests
const UNIT_TEST_TIMEOUT_SECS: u64 = 5;

/*
    Exposed enum for which state graph implementation to use
*/

#[derive(Debug, StructOpt)]
pub enum Algorithm {
    Naive,
    Simple,
    Tarjan,
    Jump,
}
impl FromStr for Algorithm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "n" | "naive" => Ok(Algorithm::Naive),
            "s" | "simple" => Ok(Algorithm::Simple),
            "t" | "tarjan" => Ok(Algorithm::Tarjan),
            "j" | "jump" => Ok(Algorithm::Jump),
            _ => Err(format!("Could not parse as Algorithm: {}", s)),
        }
    }
}
impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            Algorithm::Naive => "naive",
            Algorithm::Simple => "simple",
            Algorithm::Tarjan => "tarjan",
            Algorithm::Jump => "jump",
        };
        write!(f, "{}", result)
    }
}

/*
    Run examples with a given algorithm
*/

fn run_core(
    prefix: &str,
    alg: Algorithm,
    timeout: Duration,
    verbose: bool,
) -> ExampleResult {
    let example = Example::load_from(prefix);

    if verbose {
        println!("===== {} =====", example.name);
        println!(
            "Running algorithm '{}' with timeout {}s...",
            alg,
            timeout.as_secs()
        );
    }
    let result = match alg {
        Algorithm::Naive => {
            let mut graph = NaiveStateGraph::new();
            example.run_with_timeout(&mut graph, timeout)
        }
        Algorithm::Simple => {
            let mut graph = SimpleStateGraph::new();
            example.run_with_timeout(&mut graph, timeout)
        }
        Algorithm::Tarjan => {
            let mut graph = TarjanStateGraph::new();
            example.run_with_timeout(&mut graph, timeout)
        }
        Algorithm::Jump => {
            let mut graph = JumpStateGraph::new();
            example.run_with_timeout(&mut graph, timeout)
        }
    };

    if verbose {
        println!("=== Output ===");
        println!("{}", result.output_str());
        println!("=== Result ===");
        if result.is_correct() {
            println!("Output is correct.");
            println!("Stastics: {}", result.summary());
        } else {
            println!("Output is incorrect!");
            println!("=== Expected Output ===");
            println!("{:?}", example.expected);
        }
    } else {
        println!("{}: {}", alg, result.summary());
    }

    result
}

pub fn run_single_example(
    prefix: &str,
    algorithm: Algorithm,
    timeout_secs: u64,
) {
    let timeout = Duration::from_secs(timeout_secs);
    run_core(prefix, algorithm, timeout, true);
}

/*
    Assertion for unit testing
*/

pub fn assert_example(prefix: &str) {
    let timeout = Duration::from_secs(UNIT_TEST_TIMEOUT_SECS);
    assert!(run_core(prefix, Algorithm::Naive, timeout, true).is_correct());
    assert!(run_core(prefix, Algorithm::Simple, timeout, true).is_correct());
    assert!(run_core(prefix, Algorithm::Tarjan, timeout, true).is_correct());
    assert!(run_core(prefix, Algorithm::Jump, timeout, true).is_correct());
}

/*
    Performance comparison
*/

pub fn run_compare_csv_header() -> String {
    let header = if cfg!(debug_assertions) {
        "name, size, \
        time (naive), time (simple), time (tarjan), time (jump), \
        space (naive), space (simple), space (tarjan), space (jump)"
    } else {
        "name, size, \
        time (naive), time (simple), time (tarjan), time (jump)"
    };
    header.to_string()
}
pub fn run_compare(prefix: &str, timeout_secs: u64) -> String {
    // Returns results in CSV format

    println!("===== {} =====", prefix);
    let size = Example::load_from(prefix).len();
    println!("Example size: {}, timeout: {}s", size, timeout_secs);

    let timeout = Duration::from_secs(timeout_secs);
    let naive = run_core(prefix, Algorithm::Naive, timeout, false);
    let simple = run_core(prefix, Algorithm::Simple, timeout, false);
    let tarjan = run_core(prefix, Algorithm::Tarjan, timeout, false);
    let jump = run_core(prefix, Algorithm::Jump, timeout, false);

    let result = format!(
        "{}, {}, {}, {}, {}, {}",
        prefix,
        size,
        naive.time_str(),
        simple.time_str(),
        tarjan.time_str(),
        jump.time_str(),
    );
    if cfg!(debug_assertions) {
        format!(
            "{}, {}, {}, {}, {}",
            result,
            naive.space_str(),
            simple.space_str(),
            tarjan.space_str(),
            jump.space_str(),
        )
    } else {
        result
    }
}
