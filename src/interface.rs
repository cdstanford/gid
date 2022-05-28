/*
    Interface (Trait) for tracking live/dead states
    in an abstract transition system.

    Different data structure and algorithms for the same problem can then
    implement this interface.

    Also includes the Example struct for running specific test cases on
    an implementation of the trait.
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Live,
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
    Live(usize),
    NotReachable(usize, usize),
}

/*
    The main interface
*/

pub trait StateGraph {
    /*
        Functions that need to be implemented.

        For convenience, the main functions are unchecked:
        - add_transition_unchecked can assume both its vertices are distinct
          and that the source is Open.
        - mark_closed_unchecked can assume that its vertex is Open.
        - mark_live_unchecked can assume that its vertex is Open.
        - not_reachable_unchecked can assume that the two vertices are distinct.

        Derived checked versions are then provided as safer wrappers around
        these.
    */

    // Constructor
    fn new() -> Self
    where
        Self: Sized;

    // Add a new transition to the graph from an Open state to any state.
    // (If the vertex doesn't exist yet, create it and mark it open.)
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize);

    // Mark an open state as closed.
    fn mark_closed_unchecked(&mut self, v: usize);

    // Mark an open state as live.
    fn mark_live_unchecked(&mut self, v: usize);

    // Indicate non-reachability between two nodes.
    fn not_reachable_unchecked(&mut self, v1: usize, v2: usize);

    // Return whether v is Open, or v is Closed but there is a path from
    // v to an Open state (Unknown), or there is no such path (Dead).
    // If the state is not seen, return None.
    fn get_status(&self, v: usize) -> Option<Status>;

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

    // The safe add_transition and mark_closed, etc. should generally be used
    // over the unchecked versions as they validate that the sequence of
    // inputs is correct.
    // They also remove redundant additions that don't do anything, such
    // as a self-loop edge or marking a live state closed.
    fn add_transition(&mut self, v1: usize, v2: usize) {
        debug_assert!(self.is_open(v1) || self.is_live(v1));
        if self.is_open(v1) && v1 != v2 {
            self.add_transition_unchecked(v1, v2);
        }
    }
    fn mark_closed(&mut self, v: usize) {
        debug_assert!(self.is_open(v) || self.is_live(v));
        if self.is_open(v) {
            self.mark_closed_unchecked(v);
        }
    }
    fn mark_live(&mut self, v: usize) {
        debug_assert!(self.is_open(v) || self.is_live(v));
        if self.is_open(v) {
            self.mark_live_unchecked(v);
        }
    }
    fn not_reachable(&mut self, v1: usize, v2: usize) {
        debug_assert!(v1 != v2);
        self.not_reachable_unchecked(v1, v2);
    }

    // Some conveniences
    fn is_seen(&self, v: usize) -> bool {
        self.get_status(v).is_some()
    }
    fn is_live(&self, v: usize) -> bool {
        self.get_status(v) == Some(Status::Live)
    }
    fn is_dead(&self, v: usize) -> bool {
        self.get_status(v) == Some(Status::Dead)
    }
    fn is_unknown(&self, v: usize) -> bool {
        self.get_status(v) == Some(Status::Unknown)
    }
    fn is_open(&self, v: usize) -> bool {
        self.get_status(v).map_or(true, |st| st == Status::Open)
    }
    fn is_closed(&self, v: usize) -> bool {
        !self.is_open(v)
    }
    fn is_u_or_d(&self, v: usize) -> bool {
        self.is_dead(v) || self.is_unknown(v)
    }

    // Same as the above but using the Transaction enum
    fn process(&mut self, t: Transaction) {
        match t {
            Transaction::Add(v1, v2) => self.add_transition(v1, v2),
            Transaction::Close(v1) => self.mark_closed(v1),
            Transaction::Live(v1) => self.mark_live(v1),
            Transaction::NotReachable(v1, v2) => self.not_reachable(v1, v2),
        }
    }
}
