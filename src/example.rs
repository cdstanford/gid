/*
    Abstract the overall input/output behavior of an example run of the state
    graph interface.

    Includes interfacing with file input/output and running an example using
    a timeout.

    Core types: ExampleInput, ExampleOutput, Example, and ExampleResult.
*/

use super::constants::{EXAMPLE_EXPECT_EXT, EXAMPLE_IN_EXT};
use super::interface::{StateGraph, Status, Transaction};
use super::util;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

/*
    ExampleInput
*/

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ExampleInput(pub Vec<Transaction>);
impl ExampleInput {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn push(&mut self, t: Transaction) {
        self.0.push(t);
    }
    pub fn get_states(&self) -> HashSet<usize> {
        let mut result = HashSet::new();
        for &t in &self.0 {
            match t {
                Transaction::Add(v1, v2) => {
                    result.insert(v1);
                    result.insert(v2);
                }
                Transaction::Close(v1) => {
                    result.insert(v1);
                }
                Transaction::Live(v1) => {
                    result.insert(v1);
                }
                Transaction::NotReachable(v1, v2) => {
                    result.insert(v1);
                    result.insert(v2);
                }
            }
        }
        result
    }
}

#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExampleOutput {
    pub live: Vec<usize>,
    pub dead: Vec<usize>,
    pub unknown: Vec<usize>,
    pub open: Vec<usize>,
}
impl ExampleOutput {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add(&mut self, v: usize, st: Status) {
        match st {
            Status::Live => self.live.push(v),
            Status::Dead => self.dead.push(v),
            Status::Unknown => self.unknown.push(v),
            Status::Open => self.open.push(v),
        };
    }
    pub fn finalize(&mut self) {
        // should be called prior to saving, printing, etc.
        self.live.sort_unstable();
        self.dead.sort_unstable();
        self.unknown.sort_unstable();
        self.open.sort_unstable();
    }
}

/*
    Example struct which represents a single test case.
    Can be loaded from a file, saved to a file, or
    run to produce a results and stats object, ExampleResult.
*/

pub struct DebugStats {
    output: ExampleOutput,
    correct: bool,
    time: usize,
    space: usize,
}
pub struct ReleaseStats {
    output: ExampleOutput,
    correct: bool,
    time: Duration,
}
pub enum ExampleResult {
    Timeout,
    Debug(DebugStats),
    Release(ReleaseStats),
}
impl ExampleResult {
    pub fn is_correct(&self) -> bool {
        match self {
            Self::Timeout => false,
            Self::Debug(res) => res.correct,
            Self::Release(res) => res.correct,
        }
    }
    pub fn summary(&self) -> String {
        if let Self::Timeout = self {
            "Timeout".to_string()
        } else if !self.is_correct() {
            "Wrong Output".to_string()
        } else if let Self::Debug(res) = self {
            format!("time {}, space {}", res.time, res.space)
        } else if let Self::Release(res) = self {
            format!("time {}ms", res.time.as_millis())
        } else {
            unreachable!()
        }
    }
    pub fn time_str(&self) -> String {
        match self {
            Self::Timeout => "Timeout".to_string(),
            Self::Debug(res) => format!("{}", res.time),
            Self::Release(res) => format!("{}", res.time.as_millis()),
        }
    }
    pub fn space_str(&self) -> String {
        match self {
            Self::Timeout => "Timeout".to_string(),
            Self::Debug(res) => format!("{}", res.space),
            Self::Release(_) => "Unknown (not tracked)".to_string(),
        }
    }
    pub fn get_output(&self) -> Option<&ExampleOutput> {
        match self {
            Self::Timeout => None,
            Self::Debug(res) => Some(&res.output),
            Self::Release(res) => Some(&res.output),
        }
    }
    pub fn output_str(&self) -> String {
        match self {
            Self::Timeout => "Timeout".to_string(),
            Self::Debug(res) => format!("{:?}", res.output),
            Self::Release(res) => format!("{:?}", res.output),
        }
    }
}

fn infile_path(basename: &str) -> PathBuf {
    PathBuf::from(format!("{}{}", basename, EXAMPLE_IN_EXT))
}
fn expectfile_path(basename: &str) -> PathBuf {
    PathBuf::from(format!("{}{}", basename, EXAMPLE_EXPECT_EXT))
}
pub struct Example {
    pub basename: String, // path to example, without the extension
    pub input: ExampleInput,
    pub expected: Option<ExampleOutput>,
}
impl Example {
    pub fn new(
        basename: &str,
        input: ExampleInput,
        expected: Option<ExampleOutput>,
    ) -> Self {
        let basename = basename.to_string();
        Self { basename, input, expected }
    }
    pub fn name(&self) -> &str {
        &self.basename
    }
    pub fn len(&self) -> usize {
        self.input.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.input.0.is_empty()
    }
    pub fn load_from(basename: &str) -> Self {
        // Panics if infile (self.infile_path()) does not exist
        let infile = infile_path(basename);
        let expectfile = expectfile_path(basename);
        let input = util::from_json_file(&infile);
        let expected = if util::file_exists(&expectfile) {
            util::from_json_file(&expectfile)
        } else {
            None
        };
        Self::new(basename, input, expected)
    }
    pub fn save(&self) {
        util::to_json_file(infile_path(&self.basename), &self.input);
        if let Some(expect) = &self.expected {
            util::to_json_file(expectfile_path(&self.basename), &expect);
        }
    }

    // Run the example input on the graph, returning the output and whether
    // it matches the expected output.
    // Additionally enforces a timeout (Duration), although
    // only at the granularity of transactions.
    pub fn run_with_timeout<G: StateGraph>(
        &self,
        graph: &mut G,
        timeout: Duration,
    ) -> ExampleResult {
        let start = SystemTime::now();
        for &t in &self.input.0 {
            let time_elapsed = util::time_since(&start);
            if time_elapsed > timeout {
                return ExampleResult::Timeout;
            }
            graph.process(t);
        }
        let total_elapsed = util::time_since(&start);
        let (output, correct) = self.collect_output(graph);
        if cfg!(debug_assertions) {
            let time = graph.get_time();
            let space = graph.get_space();
            ExampleResult::Debug(DebugStats { output, correct, time, space })
        } else {
            ExampleResult::Release(ReleaseStats {
                output,
                correct,
                time: total_elapsed,
            })
        }
    }
    fn collect_output<G: StateGraph>(
        &self,
        graph: &mut G,
    ) -> (ExampleOutput, bool) {
        let mut output = ExampleOutput::new();
        for &v in self.input.get_states().iter() {
            output.add(v, graph.get_status(v).unwrap_or(Status::Open));
        }
        output.finalize();
        if let Some(expect) = &self.expected {
            let correct = &output == expect;
            (output, correct)
        } else {
            // If no expected output, just count as correct
            (output, true)
        }
    }
}
