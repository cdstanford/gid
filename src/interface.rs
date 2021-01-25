/*
    Interface (Trait) for tracking live/dead states
    in an abstract transition system.

    Different data structure and algorithms for the same problem can then
    implement this interface.

    Also includes the Example struct for running specific test cases on
    an implementation of the trait.
*/

use super::constants::{EXAMPLE_EXPECT_EXT, EXAMPLE_IN_EXT};
use super::util;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Dead,
    Unknown,
    Open,
}
impl Default for Status {
    fn default() -> Self {
        Status::Open
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Transaction {
    Add(usize, usize),
    Close(usize),
    Live(usize), // Currently a no-op
}

/*
    The main interface
*/

pub trait StateGraph: Sized {
    /*
        Functions that need to be implemented.

        For convenience, the main functions are unchecked:
        - add_transition_unchecked can assume both its vertices are distinct and that the
          source is Open.
        - mark_closed_unchecked can assume that its vertex is Open.

        Derived checked versions are then provided as safe wrappers around these.
    */

    // Constructor
    fn new() -> Self;

    // Add a new transition to the graph from an Open state to any state.
    // (If the vertex doesn't exist yet, create it and mark it open.)
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize);

    // Mark an open state as closed.
    fn mark_closed_unchecked(&mut self, v: usize);

    // Return whether v is Open, or v is Closed but there is a path from
    // v to an Open state (Unknown), or there is no such path (Dead).
    // If the state is not seen, return None.
    fn get_status(&self, v: usize) -> Option<Status>;

    // Return a vector of all states that have been seen.
    fn vec_states(&self) -> Vec<usize>;

    // Statistics -- only work in debug mode
    // space should be true memory, up to a constant, and time should be true
    // time, up to a constant.
    // Specifically: space = max sum of sizes of internal data structures;
    // time = loop iterations + recursive calls
    fn get_space(&self) -> usize;
    fn get_time(&self) -> usize;

    /*
        Derived (default) functions
    */

    // The safe add_transition and mark_closed should generally be used
    // over the unchecked versions as they validate that the sequence of
    // inputs is correct.
    fn add_transition(&mut self, v1: usize, v2: usize) {
        assert!(!self.is_closed(v1));
        if v1 != v2 {
            self.add_transition_unchecked(v1, v2);
        }
    }
    fn mark_closed(&mut self, v: usize) {
        assert!(!self.is_closed(v));
        self.mark_closed_unchecked(v);
    }
    fn mark_live(&mut self, v: usize) {
        // TODO
        self.mark_closed(v);
    }

    // Some conveniences
    fn is_seen(&self, v: usize) -> bool {
        self.get_status(v).is_some()
    }
    fn is_closed(&self, v: usize) -> bool {
        self.get_status(v).map_or(false, |st| st != Status::Open)
    }
    fn is_dead(&self, v: usize) -> bool {
        self.get_status(v) == Some(Status::Dead)
    }

    // Same as the above but using the Transaction enum
    fn process(&mut self, t: Transaction) {
        match t {
            Transaction::Add(v1, v2) => self.add_transition(v1, v2),
            Transaction::Close(v1) => self.mark_closed(v1),
            Transaction::Live(v1) => self.mark_live(v1),
        }
    }
}

/*
    Abstract the overall input/output behavior of an example
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
}

#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExampleOutput {
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
            Status::Dead => self.dead.push(v),
            Status::Unknown => self.unknown.push(v),
            Status::Open => self.open.push(v),
        };
    }
    pub fn finalize(&mut self) {
        // should be called prior to saving, printing, etc.
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
        for &v in &graph.vec_states() {
            output.add(v, graph.get_status(v).unwrap());
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
