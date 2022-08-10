/*
    The driver:
    Handles running a series of transactions using the state
    graph interface, and collecting/viewing/checking the output.
*/

use super::algorithm::{
    BFGTStateGraph, JumpStateGraph, NaiveStateGraph, OptimizedStateGraph,
    PolylogStateGraph, SimpleStateGraph,
};
use super::constants::EXAMPLE_IN_EXT;
use super::example::{Example, ExampleOutput, ExampleResult};
use super::interface::StateGraph;
use super::util;
use std::fmt::{self, Debug};
use std::ops::DerefMut;
use std::str::FromStr;
use std::time::Duration;

/*
    Exposed enum for which state graph implementation to use
*/

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    Naive,
    Simple,
    BFGT,
    Jump,
    Polylog,
    Optimized,
}
impl FromStr for Algorithm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "n" | "naive" => Ok(Algorithm::Naive),
            "s" | "simple" => Ok(Algorithm::Simple),
            "b" | "bfgt" => Ok(Algorithm::BFGT),
            "j" | "jump" => Ok(Algorithm::Jump),
            "p" | "polylog" => Ok(Algorithm::Polylog),
            "o" | "optimized" => Ok(Algorithm::Optimized),
            _ => Err(format!("Could not parse as Algorithm: {}", s)),
        }
    }
}
impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match self {
            Algorithm::Naive => "naive",
            Algorithm::Simple => "simple",
            Algorithm::BFGT => "bfgt",
            Algorithm::Jump => "jump",
            Algorithm::Polylog => "polylog",
            Algorithm::Optimized => "optimized",
        };
        write!(f, "{}", result)
    }
}
impl Algorithm {
    fn new_graph(&self) -> Box<dyn StateGraph> {
        match self {
            Algorithm::Naive => Box::new(NaiveStateGraph::new()),
            Algorithm::Simple => Box::new(SimpleStateGraph::new()),
            Algorithm::BFGT => Box::new(BFGTStateGraph::new()),
            Algorithm::Jump => Box::new(JumpStateGraph::new()),
            Algorithm::Polylog => Box::new(PolylogStateGraph::new()),
            Algorithm::Optimized => Box::new(OptimizedStateGraph::new()),
        }
    }
}

/*
    Run examples with a given algorithm
*/

fn run_core(
    example: &Example,
    alg: Algorithm,
    timeout: Duration,
    verbose: bool,
) -> ExampleResult {
    if verbose {
        println!("===== {} =====", example.name());
        println!(
            "Running algorithm '{}' with timeout {}s...",
            alg,
            timeout.as_secs()
        );
    }
    let mut graph = alg.new_graph();
    let result = example.run_with_timeout(graph.deref_mut(), timeout);

    if verbose {
        println!("=== Output ===");
        println!("{}", result.output_str());
        println!("=== Result ===");
        println!("Stastics: {}", result.summary());
        if example.expected.is_some() {
            if result.is_correct() {
                println!("Output is correct.");
            } else {
                println!("Output is incorrect!");
                println!("=== Expected Output ===");
                println!("{:?}", example.expected.as_ref().unwrap());
            }
        }
    } else {
        println!("{}: {}", alg, result.summary());
    }

    result
}

pub fn run_single_example(
    basename: &str,
    algorithm: Algorithm,
    timeout_secs: u64,
) {
    let example = Example::load_from(basename);
    let timeout = Duration::from_secs(timeout_secs);
    run_core(&example, algorithm, timeout, true);
}

/*
    Lists of algorithms
*/
pub const ALL_ALGS: &[Algorithm] = &[
    Algorithm::Naive,
    Algorithm::Simple,
    Algorithm::BFGT,
    Algorithm::Jump,
    Algorithm::Polylog,
];
pub fn algs_excluding(exclude: &[Algorithm]) -> Vec<Algorithm> {
    ALL_ALGS.iter().filter(|&x| !exclude.contains(x)).cloned().collect()
}
pub fn algs_all() -> Vec<Algorithm> {
    algs_excluding(&[])
}

/*
    Assertion for unit testing
*/

// Panic on timeouts, otherwise extract output
pub fn unwrap_timeout(res: &ExampleResult) -> &ExampleOutput {
    res.get_output().unwrap_or_else(|| {
        panic!(
            "Algorithm timed out! Tip: if you are running \
            --ignored tests, make sure to use --release mode."
        );
    })
}

pub fn assert_example(basename: &str, timeout_secs: Option<u64>) {
    let example = Example::load_from(basename);
    let timeout_secs = timeout_secs.unwrap_or(u64::MAX);
    let timeout = Duration::from_secs(timeout_secs);
    let algs = algs_all();

    // If example has expected output, check each algorithm is correct
    // separately. Otherwise, compare them with respect to each other.
    if example.expected.is_some() {
        println!("Asserting each algorithm output matches expected...");
        for alg in algs {
            let out = run_core(&example, alg, timeout, true);
            assert!(out.is_correct());
        }
    } else {
        assert!(!algs.is_empty());
        println!("Asserting each algorithm output matches {}...", algs[0]);
        let out = run_core(&example, algs[0], timeout, true);
        let expected = unwrap_timeout(&out);
        for &alg in algs.iter().skip(1) {
            let out = run_core(&example, alg, timeout, true);
            assert_eq!(expected, unwrap_timeout(&out));
        }
    }
}

/*
    Performance comparison
*/

pub fn run_compare_csv_header(algs: &[Algorithm]) -> String {
    let mut header = "name, size, timeout".to_string();
    for alg in algs {
        header += &format!(", time ({})", alg);
        if cfg!(debug_assertions) {
            header += &format!(", space ({})", alg);
        }
    }
    header
}
pub fn run_compare(
    basename: &str,
    algs: &[Algorithm],
    timeout_secs: u64,
) -> String {
    // Returns results in CSV format

    let example = Example::load_from(basename);
    println!("===== {} =====", example.name());
    println!("Example size: {}, timeout: {}s", example.len(), timeout_secs);
    let mut result =
        format!("{}, {}, {}", example.name(), example.len(), timeout_secs);

    let timeout = Duration::from_secs(timeout_secs);
    for &alg in algs {
        let out = run_core(&example, alg, timeout, false);
        result += &format!(", {}", out.time_str());
        if cfg!(debug_assertions) {
            result += &format!(", {}", out.space_str());
        }
    }

    // Uncomment to debug the CSV output to make sure it matches
    // println!("{}", result);
    // println!("{}", run_compare_csv_header(algs));

    result
}

/*
    Get all example basenames in a directory
*/

pub fn example_basenames_in_dir(dir: &str) -> Vec<String> {
    util::paths_in(dir)
        .filter_map(|path| path.strip_suffix(EXAMPLE_IN_EXT).map(String::from))
        .collect()
}
