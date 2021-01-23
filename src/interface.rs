/*
    Interface (Trait) for tracking live/dead states
    in an abstract transition system.

    Different data structure and algorithms for the same problem can then
    implement this interface.
*/

use super::util;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    Example struct: represents a single test case, which can be read from a file
    or saved to a file
*/
fn infile_from_prefix(prefix: &str) -> String {
    format!("examples/{}_in.json", prefix)
}
fn expectedfile_from_prefix(prefix: &str) -> String {
    format!("examples/{}_out.json", prefix)
}
pub struct Example(pub String, pub ExampleInput, pub ExampleOutput);
impl Example {
    pub fn new(
        prefix: &str,
        input: ExampleInput,
        output: ExampleOutput,
    ) -> Self {
        Self(prefix.to_string(), input, output)
    }
    pub fn load_from(prefix: &str) -> Self {
        // May panic if file(s) do not exist
        let infile = PathBuf::from(infile_from_prefix(prefix));
        let outfile = PathBuf::from(expectedfile_from_prefix(prefix));
        let input = util::from_json_file(&infile);
        let output = util::from_json_file(&outfile);
        Self::new(prefix, input, output)
    }
    pub fn save(&self) {
        util::to_json_file(infile_from_prefix(&self.0), &self.1);
        util::to_json_file(expectedfile_from_prefix(&self.0), &self.2);
    }

    // Run the example input on the graph, returning the output and whether
    // it matches the expected output.
    pub fn run<G: StateGraph>(&self, graph: &mut G) -> (ExampleOutput, bool) {
        for &t in &self.1 .0 {
            graph.process(t);
        }
        let mut result = ExampleOutput::new();
        for &v in &graph.vec_states() {
            result.add(v, graph.get_status(v).unwrap());
        }
        result.finalize();
        let matches = result == self.2;
        (result, matches)
    }
}
