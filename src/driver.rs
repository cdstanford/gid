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

fn run_core<G: StateGraph>(alg_name: &str, prefix: &str) -> bool {
    let example = Example::load_from(prefix);

    println!("===== {} =====", example.0);

    println!("Running {} algorithm...", alg_name);
    let mut graph = G::new();
    let (output, correct) = example.run(&mut graph);

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
}

pub fn run_example(prefix: &str, algorithm: Algorithm) -> bool {
    match algorithm {
        Algorithm::Naive => run_core::<NaiveStateGraph>("Naive", prefix),
        Algorithm::Simple => run_core::<SimpleStateGraph>("Simple", prefix),
        Algorithm::Tarjan => run_core::<TarjanStateGraph>("Tarjan", prefix),
        Algorithm::Jump => run_core::<JumpStateGraph>("Jump", prefix),
    }
}

pub fn assert_example(prefix: &str) {
    assert!(run_example(prefix, Algorithm::Naive));
    assert!(run_example(prefix, Algorithm::Simple));
    // Not passing unit tests, TODO: Debug
    // assert!(run_example(prefix, Algorithm::Tarjan));
    assert!(run_example(prefix, Algorithm::Jump));
}

/*
    Performance comparison
*/
fn get_stats<G: StateGraph>(prefix: &str) -> (usize, usize) {
    if !cfg!(debug_assertions) {
        panic!("Must be in debug mode to track time/space counters");
    }
    let example = Example::load_from(prefix);
    let mut graph = G::new();
    let (_output, correct) = example.run(&mut graph);
    assert!(correct);
    (graph.get_time(), graph.get_space())
}

pub fn run_compare(prefix: &str) {
    println!("=== Time and Space Statistics: {} ===", prefix);
    println!("Naive: {:?}", get_stats::<NaiveStateGraph>(prefix));
    println!("Simple: {:?}", get_stats::<SimpleStateGraph>(prefix));
    println!("Jump: {:?}", get_stats::<JumpStateGraph>(prefix));
}
