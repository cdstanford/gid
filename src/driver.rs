/*
    The driver:
    Handles running a series of transactions using the state
    graph interface, and collecting/viewing/checking the output.
*/

use super::algorithm::{
    JumpStateGraph, NaiveStateGraph, SimpleStateGraph, TarjanStateGraph,
};
use super::interface::{Example, StateGraph};
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
    Run examples or test cases
*/

fn run_core<G: StateGraph>(
    alg_name: &str,
    prefix: &str,
    timeout: Duration,
) -> bool {
    let example = Example::load_from(prefix);

    println!("===== {} =====", example.0);

    println!("Running {} algorithm...", alg_name);
    let mut graph = G::new();
    let result = example.run_with_timeout(&mut graph, timeout);

    if let Some((output, correct)) = result {
        if cfg!(debug_assertions) {
            println!("=== Statistics ===");
            println!("Time: {}", graph.get_time());
            println!("Space: {}", graph.get_space());
        }

        println!("=== Output ===");
        println!("{:?}", output);

        if correct {
            println!("Success: output is as expected");
            true
        } else {
            println!("Failed: output is incorrect!");
            println!("=== Expected ===");
            println!("{:?}", example.2);
            false
        }
    } else {
        println!("Timed Out");
        false
    }
}

pub fn run_example(
    prefix: &str,
    algorithm: Algorithm,
    timeout_secs: u64,
) -> bool {
    let timeout = Duration::from_secs(timeout_secs);
    match algorithm {
        Algorithm::Naive => {
            run_core::<NaiveStateGraph>("Naive", prefix, timeout)
        }
        Algorithm::Simple => {
            run_core::<SimpleStateGraph>("Simple", prefix, timeout)
        }
        Algorithm::Tarjan => {
            run_core::<TarjanStateGraph>("Tarjan", prefix, timeout)
        }
        Algorithm::Jump => run_core::<JumpStateGraph>("Jump", prefix, timeout),
    }
}

pub fn assert_example(prefix: &str) {
    let timeout_secs = UNIT_TEST_TIMEOUT_SECS;
    assert!(run_example(prefix, Algorithm::Naive, timeout_secs));
    assert!(run_example(prefix, Algorithm::Simple, timeout_secs));
    assert!(run_example(prefix, Algorithm::Tarjan, timeout_secs));
    assert!(run_example(prefix, Algorithm::Jump, timeout_secs));
}

/*
    Performance comparison
*/
fn get_stats<G: StateGraph>(
    prefix: &str,
    timeout: Duration,
) -> Option<(usize, usize)> {
    if !cfg!(debug_assertions) {
        panic!("Must be in debug mode to track time/space counters");
    }
    let example = Example::load_from(prefix);
    let mut graph = G::new();
    example.run_with_timeout(&mut graph, timeout).map(|(_output, correct)| {
        assert!(correct);
        (graph.get_time(), graph.get_space())
    })
}
fn format_stats<G: StateGraph>(prefix: &str, timeout: Duration) -> String {
    if let Some((time, space)) = get_stats::<G>(prefix, timeout) {
        format!("time {}, space {}", time, space)
    } else {
        "Timeout".to_string()
    }
}
pub fn run_compare(prefix: &str, timeout_secs: u64) {
    let timeout = Duration::from_secs(timeout_secs);
    println!("=== Debug Counter Statistics: {} ===", prefix);
    println!("Naive: {}", format_stats::<NaiveStateGraph>(prefix, timeout));
    println!("Simple: {}", format_stats::<SimpleStateGraph>(prefix, timeout));
    println!("Tarjan: {}", format_stats::<TarjanStateGraph>(prefix, timeout));
    println!("Jump: {}", format_stats::<JumpStateGraph>(prefix, timeout));
}
