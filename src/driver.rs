/*
    The driver:
    Handles running a series of transactions using the state
    graph interface, and collecting/viewing/checking the output.
*/

use super::algorithm::{
    JumpStateGraph, NaiveStateGraph, PolylogStateGraph, SimpleStateGraph,
    TarjanStateGraph,
};
use super::constants::EXAMPLE_IN_EXT;
use super::example::{Example, ExampleOutput, ExampleResult};
use super::interface::StateGraph;
use std::fmt::{self, Debug};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use structopt::StructOpt;

/*
    Exposed enum for which state graph implementation to use
*/

#[derive(Debug, StructOpt)]
pub enum Algorithm {
    Naive,
    Simple,
    Tarjan,
    Jump,
    Polylog,
}
impl FromStr for Algorithm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "n" | "naive" => Ok(Algorithm::Naive),
            "s" | "simple" => Ok(Algorithm::Simple),
            "t" | "tarjan" => Ok(Algorithm::Tarjan),
            "j" | "jump" => Ok(Algorithm::Jump),
            "p" | "polylog" => Ok(Algorithm::Polylog),
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
            Algorithm::Polylog => "polylog",
        };
        write!(f, "{}", result)
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
        Algorithm::Polylog => {
            let mut graph = PolylogStateGraph::new();
            example.run_with_timeout(&mut graph, timeout)
        }
    };

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

pub fn assert_example(basename: &str, timeout_secs: u64) {
    let example = Example::load_from(basename);
    let timeout = Duration::from_secs(timeout_secs);

    // If example has expected output, check each algorithm is correct
    // separately. Otherwise, compare them with respect to each other.
    if example.expected.is_some() {
        println!("Asserting each algorithm output matches expected...");
        let naive = run_core(&example, Algorithm::Naive, timeout, true);
        assert!(naive.is_correct());
        let simple = run_core(&example, Algorithm::Simple, timeout, true);
        assert!(simple.is_correct());
        let tarjan = run_core(&example, Algorithm::Tarjan, timeout, true);
        assert!(tarjan.is_correct());
        let jump = run_core(&example, Algorithm::Jump, timeout, true);
        assert!(jump.is_correct());
        let polylog = run_core(&example, Algorithm::Polylog, timeout, true);
        assert!(polylog.is_correct());
    } else {
        println!("Asserting each algorithm output matches naive...");
        let naive = run_core(&example, Algorithm::Naive, timeout, true);
        let expected = unwrap_timeout(&naive);
        let simple = run_core(&example, Algorithm::Simple, timeout, true);
        assert_eq!(expected, unwrap_timeout(&simple));
        let tarjan = run_core(&example, Algorithm::Tarjan, timeout, true);
        assert_eq!(expected, unwrap_timeout(&tarjan));
        let jump = run_core(&example, Algorithm::Jump, timeout, true);
        assert_eq!(expected, unwrap_timeout(&jump));
        let polylog = run_core(&example, Algorithm::Polylog, timeout, true);
        assert!(polylog.is_correct());
    }
}

/*
    Performance comparison
*/

pub fn run_compare_csv_header() -> String {
    let header = if cfg!(debug_assertions) {
        "name, size, \
        time (naive), time (simple), time (tarjan), time (jump), \
        time (polylog), \
        space (naive), space (simple), space (tarjan), space (jump), \
        space (polylog)"
    } else {
        "name, size, \
        time (naive), time (simple), time (tarjan), time (jump), \
        time (polylog)"
    };
    header.to_string()
}
pub fn run_compare(basename: &str, timeout_secs: u64) -> String {
    // Returns results in CSV format

    let example = Example::load_from(basename);
    println!("===== {} =====", example.name());
    println!("Example size: {}, timeout: {}s", example.len(), timeout_secs);

    let timeout = Duration::from_secs(timeout_secs);
    let naive = run_core(&example, Algorithm::Naive, timeout, false);
    let simple = run_core(&example, Algorithm::Simple, timeout, false);
    let tarjan = run_core(&example, Algorithm::Tarjan, timeout, false);
    let jump = run_core(&example, Algorithm::Jump, timeout, false);
    let polylog = run_core(&example, Algorithm::Polylog, timeout, false);

    let result = format!(
        "{}, {}, {}, {}, {}, {}, {}",
        example.name(),
        example.len(),
        naive.time_str(),
        simple.time_str(),
        tarjan.time_str(),
        jump.time_str(),
        polylog.time_str(),
    );
    if cfg!(debug_assertions) {
        format!(
            "{}, {}, {}, {}, {}, {}",
            result,
            naive.space_str(),
            simple.space_str(),
            tarjan.space_str(),
            jump.space_str(),
            polylog.space_str(),
        )
    } else {
        result
    }
}

/*
    Get all example basenames in a directory

    It is truly hilarious how many layers of indirection it takes
    to go through Rust's various String and OS abstractions
*/

pub fn example_basenames_in_dir(dir: &str) -> Vec<String> {
    fs::read_dir(PathBuf::from(dir))
        .unwrap_or_else(|err| {
            panic!("couldn't view files in directory: {} ({})", dir, err)
        })
        .map(|file| {
            file.unwrap_or_else(|err| {
                panic!("error viewing file in directory: {} ({})", dir, err)
            })
        })
        .map(|file| file.path().into_os_string())
        .map(|osstr| {
            osstr.into_string().unwrap_or_else(|err| {
                panic!("found file path with invalid unicode ({:?})", err)
            })
        })
        .map(|path| path.strip_suffix(EXAMPLE_IN_EXT).map(String::from))
        .flatten()
        .collect()
}
