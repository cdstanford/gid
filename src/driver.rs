/*
    The driver:
    - handles file I/O, running a series of transactions using the state
      graph interface, and collecting/viewing/checking the output.
*/

use super::interface::{ExampleInput, ExampleOutput, StateGraph};
use super::naive::NaiveStateGraph;
use super::simple::SimpleStateGraph;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use structopt::StructOpt;

/*
    Exposed enum for which state graph implementation to use
*/

#[derive(Debug, StructOpt)]
pub enum Algorithm {
    Naive,
    Simple,
    // Tarjan,
    // Smart,
}
impl FromStr for Algorithm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "n" | "naive" => Ok(Algorithm::Naive),
            "s" | "simple" => Ok(Algorithm::Simple),
            _ => Err(format!("Could not parse as Algorithm: {}", s)),
        }
    }
}

/*
    Utility for file I/O
*/

pub fn path_reader<P>(path: P) -> BufReader<File>
where
    P: AsRef<Path> + Debug,
{
    BufReader::new(File::open(&path).unwrap_or_else(|err| {
        panic!("Could not open file at: {:?} -- {}", path, err)
    }))
}

pub fn from_json_file<P, T>(path: P) -> T
where
    P: AsRef<Path> + Debug,
    T: DeserializeOwned,
{
    serde_json::from_reader(path_reader(&path)).unwrap_or_else(|err| {
        panic!("Could not read JSON: {:?} -- {}", path, err)
    })
}

/*
    Main driver to run examples
*/

fn run_core<G: StateGraph>(
    in_file: &PathBuf,
    expected_out_file: Option<&PathBuf>,
    alg_name: &str,
) -> bool {
    let input: ExampleInput = from_json_file(in_file);
    let expect: Option<ExampleOutput> = expected_out_file.map(from_json_file);

    println!("===== State Graph =====");

    println!("Running {} algorithm...", alg_name);
    let mut graph = G::new();
    graph.process_all(&input);
    let output = graph.collect_all();

    if cfg!(debug_assertions) {
        println!("=== Statistics ===");
        println!("Time: {}", graph.get_time());
        println!("Space: {}", graph.get_space());
    }

    println!("=== Output ===");
    println!("{:?}", output);

    if let Some(expected) = expect {
        if output == expected {
            println!("Success: output is as expected");
            true
        } else {
            println!("Failed: output is incorrect!");
            println!("=== Expected ===");
            println!("{:?}", expected);
            false
        }
    } else {
        true
    }
}

pub fn run_example(
    in_file: &PathBuf,
    expected_out_file: Option<&PathBuf>,
    algorithm: Algorithm,
) -> bool {
    match algorithm {
        Algorithm::Naive => {
            run_core::<NaiveStateGraph>(in_file, expected_out_file, "Naive")
        }
        Algorithm::Simple => {
            run_core::<SimpleStateGraph>(in_file, expected_out_file, "Simple")
        }
    }
}

/*
    Unit tests from the input/output files
*/
#[cfg(test)]
mod tests {
    use super::*;

    fn test_file(prefix: &str) {
        let infile = PathBuf::from(format!("examples/{}_in.json", prefix));
        let outfile = PathBuf::from(format!("examples/{}_out.json", prefix));
        assert!(run_example(&infile, Some(&outfile), Algorithm::Naive));
        // TODO Uncomment and get test working
        assert!(run_example(&infile, Some(&outfile), Algorithm::Simple));
    }

    #[test]
    fn test_1() {
        test_file("1");
    }
    #[test]
    fn test_line_10() {
        test_file("line_10");
    }
    #[test]
    fn test_reverseline_10() {
        test_file("reverseline_10");
    }
    #[test]
    fn test_line_20() {
        test_file("line_20");
    }
    #[test]
    fn test_reverseline_20() {
        test_file("reverseline_20");
    }
    #[test]
    fn test_tree_3() {
        test_file("tree_3");
    }
    #[test]
    fn test_loop_3() {
        test_file("loop_3");
    }
    #[test]
    fn test_liveloop_3() {
        test_file("liveloop_3");
    }
    #[test]
    fn test_reverseloop_3() {
        test_file("reverseloop_3");
    }
}
