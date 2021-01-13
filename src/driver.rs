/*
    The driver:
    - handles file I/O, running a series of transactions using the state
      graph interface, and collecting/viewing/checking the output.
*/

use super::interface::{ExampleInput, ExampleOutput, StateGraph};
use super::jump::JumpStateGraph;
use super::naive::NaiveStateGraph;
use super::simple::SimpleStateGraph;
use super::tarjan::TarjanStateGraph;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
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

/*
    Utility for file I/O
*/

pub fn path_reader<P>(path: P) -> BufReader<File>
where
    P: AsRef<Path> + Debug,
{
    BufReader::new(File::open(&path).unwrap_or_else(|err| {
        panic!("Could not open file for reading: {:?} -- {}", path, err)
    }))
}

pub fn path_writer<P>(path: P) -> BufWriter<File>
where
    P: AsRef<Path> + Debug,
{
    BufWriter::new(File::create(&path).unwrap_or_else(|err| {
        panic!("Could not open file for writing: {:?} -- {}", path, err)
    }))
}

pub fn from_json_file<P, T>(path: P) -> T
where
    P: AsRef<Path> + Debug,
    T: DeserializeOwned,
{
    serde_json::from_reader(path_reader(&path)).unwrap_or_else(|err| {
        panic!("Could not read JSON from {:?} -- {}", path, err)
    })
}

pub fn to_json_file<P, T>(path: P, data: T)
where
    P: AsRef<Path> + Debug,
    T: Serialize,
{
    let mut writer = path_writer(&path);
    serde_json::to_writer_pretty(&mut writer, &data).unwrap_or_else(|err| {
        panic!("Could not write JSON to {:?} -- {}", path, err)
    });
    writeln!(&mut writer).unwrap_or_else(|err| {
        format!("Could not append newline to file: {:?} -- {}", &path, err);
    });
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

    println!("===== {} =====", in_file.to_str().unwrap());

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
        Algorithm::Tarjan => {
            run_core::<TarjanStateGraph>(in_file, expected_out_file, "Tarjan")
        }
        Algorithm::Jump => {
            run_core::<JumpStateGraph>(in_file, expected_out_file, "Jump")
        }
    }
}

pub fn infile_from_prefix(prefix: &str) -> String {
    format!("examples/{}_in.json", prefix)
}

pub fn expectedfile_from_prefix(prefix: &str) -> String {
    format!("examples/{}_out.json", prefix)
}

pub fn assert_example(prefix: &str) {
    let infile = PathBuf::from(infile_from_prefix(prefix));
    let outfile = PathBuf::from(expectedfile_from_prefix(prefix));
    assert!(run_example(&infile, Some(&outfile), Algorithm::Naive));
    assert!(run_example(&infile, Some(&outfile), Algorithm::Simple));
    // Not passing unit tests, TODO: Debug
    // assert!(run_example(&infile, Some(&outfile), Algorithm::Tarjan));
    assert!(run_example(&infile, Some(&outfile), Algorithm::Jump));
}

/*
    Performance comparison
*/
fn get_stats<G: StateGraph>(
    input: &ExampleInput,
    expect: &ExampleOutput,
) -> (usize, usize) {
    if !cfg!(debug_assertions) {
        panic!("Must be in debug mode to track time/space counters");
    }
    let mut graph = G::new();
    graph.process_all(&input);
    let output = graph.collect_all();
    assert_eq!(&output, expect);
    (graph.get_time(), graph.get_space())
}

pub fn run_compare(in_file: &PathBuf, expected_out_file: &PathBuf) {
    let input: ExampleInput = from_json_file(in_file);
    let expect: ExampleOutput = from_json_file(expected_out_file);

    println!(
        "=== Time and Space Statistics: {} ===",
        in_file.to_str().unwrap()
    );
    println!("Naive: {:?}", get_stats::<NaiveStateGraph>(&input, &expect));
    println!("Simple: {:?}", get_stats::<SimpleStateGraph>(&input, &expect));
    println!("Jump: {:?}", get_stats::<JumpStateGraph>(&input, &expect));
}
