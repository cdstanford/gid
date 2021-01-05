/*
    Interface (Trait) for tracking live/dead states
    in an abstract transition system.

    Different data structure and algorithms for the same problem can then
    implement this interface.
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Dead,
    Unknown,
    Unvisited,
}
impl Default for Status {
    fn default() -> Self {
        Status::Unvisited
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Transaction {
    Add(usize, usize),
    Done(usize),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ExampleInput(pub Vec<Transaction>);
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExampleOutput {
    dead: Vec<usize>,
    unknown: Vec<usize>,
    unvisited: Vec<usize>,
}

pub trait StateGraph: Sized {
    /*
        Functions that need to be implemented.

        For convenience, the main functions are unchecked:
        - add_transition_unchecked can assume both its vertices are distinct and that the
          source is Unvisited.
        - mark_done_unchecked can assume that its vertex is Unvisited.

        Derived checked versions are then provided as safe wrappers around these.
    */

    // Constructor
    fn new() -> Self;

    // Add a new transition to the graph from an Unvisited state to any state.
    // (If the vertex doesn't exist yet, create it and mark it unvisited.)
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize);

    // Mark an unvisited state as visited.
    fn mark_done_unchecked(&mut self, v: usize);

    // Return whether v is Unvisited, or v is Visited but there is a path from
    // v to an Unvisited state (Unknown), or there is no such path (Dead).
    fn get_status(&self, v: usize) -> Status;

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

    // The safe add_transition and mark_done should generally be used
    // over the unchecked versions as they validate that the sequence of
    // inputs is correct.
    fn add_transition(&mut self, v1: usize, v2: usize) {
        assert!(!self.is_done(v1));
        if v1 != v2 {
            self.add_transition_unchecked(v1, v2);
        }
    }
    fn mark_done(&mut self, v: usize) {
        assert!(!self.is_done(v));
        self.mark_done_unchecked(v);
    }

    // Some conveniences
    fn is_done(&self, v: usize) -> bool {
        self.get_status(v) != Status::Unvisited
    }
    fn is_dead(&self, v: usize) -> bool {
        self.get_status(v) == Status::Dead
    }

    // Same as the above but using the Transaction enum
    fn process(&mut self, t: Transaction) {
        match t {
            Transaction::Add(v1, v2) => self.add_transition(v1, v2),
            Transaction::Done(v1) => self.mark_done(v1),
        }
    }
    fn process_all(&mut self, ts: &ExampleInput) {
        for &t in &ts.0 {
            self.process(t);
        }
    }

    // Final output
    // Return the list of states of each status, sorted.
    fn collect_all(&self) -> ExampleOutput {
        let mut dead = Vec::new();
        let mut unknown = Vec::new();
        let mut unvisited = Vec::new();
        for &v in &self.vec_states() {
            match self.get_status(v) {
                Status::Dead => dead.push(v),
                Status::Unknown => unknown.push(v),
                Status::Unvisited => unvisited.push(v),
            };
        }
        dead.sort_unstable();
        unknown.sort_unstable();
        unvisited.sort_unstable();
        ExampleOutput { dead, unknown, unvisited }
    }

    // Top-level function to run an input and produce an output
    fn run_example(input: &ExampleInput) -> ExampleOutput {
        let mut state_graph = Self::new();
        state_graph.process_all(input);
        state_graph.collect_all()
    }
}
