/*
    Our new implementation of the StateGraph trait.
    Uses "jump" pointers to jump from each state a large number
    of states ahead at once.

    TODO (Planned)
*/

use super::debug_counter::DebugCounter;
use super::graph::DiGraph;
use super::interface::{StateGraph, Status};

#[derive(Debug, Default, PartialEq)]
struct Node {
    jumps: Vec<usize>,
    reserve_edges: Vec<usize>,
    status: Status,
}

#[derive(Debug, Default)]
pub struct JumpStateGraph {
    graph: DiGraph<usize, Node>,
    additional_time: DebugCounter,
}
impl JumpStateGraph {
    fn set_status(&mut self, v: usize, status: Status) {
        debug_assert!(self.graph.is_seen(v));
        self.graph.get_label_mut(v).unwrap().status = status;
    }
    fn get_last_jump(&self, v: usize) -> usize {
        // Get the current last element in the jumps list.
        debug_assert!(self.is_done(v));
        debug_assert!(!self.graph.get_label(v).unwrap().jumps.is_empty());
        *self.graph.get_label(v).unwrap().jumps.last().unwrap()
    }
    fn pop_last_jump(&mut self, v: usize) {
        // Remove the current last element in the jumps list.
        debug_assert!(self.is_done(v));
        debug_assert!(!self.graph.get_label(v).unwrap().jumps.is_empty());
        self.graph.get_label_mut(v).unwrap().jumps.pop();
    }
    fn jump(&mut self, v: usize) -> usize {
        // Jump from v to the Univisted vertex it currently points to.
        // The assumption / invariant is that although some jumps
        // may be obsolete, there is always an unvisited vertex that
        // is pointed to once obsolete jumps are removed.
        debug_assert!(self.graph.is_seen(v));
        if !self.is_dead(v) {
            return v;
        }
        while self.is_dead(self.get_last_jump(v)) {
            self.pop_last_jump(v);
        }
        self.jump(self.get_last_jump(v))
        // TODO: Update jumps list
    }
}
impl StateGraph for JumpStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        self.graph.ensure_edge(v1, v2);
    }
    fn mark_done_unchecked(&mut self, v: usize) {
        self.set_status(v, Status::Unknown);
        // TODO: Do something here
    }
    fn get_status(&self, v: usize) -> Status {
        self.graph.get_label_or_default(v).status
    }
    fn vec_states(&self) -> Vec<usize> {
        self.graph.iter_vertices_all().collect()
    }
    fn get_space(&self) -> usize {
        self.graph.get_space()
    }
    fn get_time(&self) -> usize {
        self.graph.get_time()
    }
}
