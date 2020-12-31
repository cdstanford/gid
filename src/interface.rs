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

pub trait StateGraph {
    /*
        Functions that need to be implemented.

        For convenience, these versions are unchecked:
        - add_transition_unchecked can assume both its vertices are distinct and that the
          source is Unvisited.
        - mark_done_unchecked can assume that its vertex is Unvisited.
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

    // Safe, checked versions
    // These should always be used over the unchecked versions for code that
    // uses the interface naively.
    // TODO: If this becomes a real library, return Result instead of panicking.
    fn add_transition(&mut self, v1: usize, v2: usize) {
        assert!(self.get_status(v1) == Status::Unvisited);
        if v1 != v2 {
            self.add_transition_unchecked(v1, v2);
        }
    }
    fn mark_done(&mut self, v: usize) {
        assert!(self.get_status(v) == Status::Unvisited);
        self.mark_done_unchecked(v);
    }

    // Same as the above but using the Transaction enum
    fn process_unchecked(&mut self, t: Transaction) {
        match t {
            Transaction::Add(v1, v2) => self.add_transition_unchecked(v1, v2),
            Transaction::Done(v1) => self.mark_done_unchecked(v1),
        }
    }
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
    fn run_example(input: &ExampleInput) -> ExampleOutput
    where
        Self: Sized,
    {
        let mut state_graph = Self::new();
        state_graph.process_all(input);
        state_graph.collect_all()
    }
}

// impl FromStr for Transaction {
//     type Err = String;
//     fn from_str(s: &str) -> Result<Self, String> {
//         let parts: Vec<String> = s.split(' ').collect();
//         if parts[0] == "Add" {
//             if parts.len() == 3 {
//                 let arg1 = parts[1]
//                     .parse::<usize>()
//                     .map_err(|err| format!(
//                         "{}: Could not parse Add arg as int: {}",
//                         err, parts[1]
//                     ))?;
//                 let arg2 = parts[2]
//                     .parse::<usize>()
//                     .map_err(|err| format!(
//                         "{}: Could not parse Add arg as int: {}",
//                         err, parts[2]
//                     ))?;
//                 Transaction::Add(arg1, arg2)
//             } else {
//                 Error("Wrong number of arguments to Add")
//             }
//         } else if parts[0] == "Done" {
//             if parts.len() == 2 {
//                 let arg1 = parts[1]
//                     .parse::<usize>()
//                     .map_err(|err| format!(
//                         "{}: Could not parse Add arg as int: {}",
//                         err, parts[1]
//                     ))?;
//                 Transaction::Done(arg1)
//             } else {
//                 Error("Wrong number of arguments to Add")
//             }
//         } else {
//             Error(format!("Unrecognized transaction: {:?}", parts))
//         }
//     }
// }
