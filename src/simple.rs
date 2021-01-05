/*
    Slightly less naive implemenation implementing the
    state graph interface, this time using Union Find.

    Differs from naive, which doesn't use Union Find (doesn't merge SCCs).

    Parallels state_graph.cpp in the Z3 code base -- found in
        z3/src/util/state_graph.h
    with one improvement: we used LinkedList instead of HashSet for
    storing edges, because it allows merging edge sets in O(1).
    (see graph.rs)
*/

use super::debug_counter::DebugCounter;
use super::graph::DiGraph;
use super::interface::{StateGraph, Status};

#[derive(Debug, Default)]
pub struct SimpleStateGraph {
    graph: DiGraph<usize, Status>,
    // Debug mode statistics -- on top of those tracked by graph
    additional_time: DebugCounter,
}
impl SimpleStateGraph {
    fn merge_all_cycles(&mut self, v: usize) {
        debug_assert!(self.is_done(v));
        // Merge all cycles through v (assuming no other cycles in Done states)
        // TODO
    }
    fn check_dead_recursive(&mut self, v: usize) {
        // Check if v is dead and recurse on back edges.

        // If v is already dead or not dead, return.
        if self.is_dead(v) {
            return;
        }
        for w in self.graph.iter_fwd_edges(v) {
            if !self.is_dead(w) {
                return;
            }
        }
        // Mark v dead
        self.graph.overwrite_vertex(v, Status::Dead);
        // Recurse
        let mut to_recurse: Vec<usize> = self.graph.iter_bck_edges(v).collect();
        for u in to_recurse.drain(..) {
            self.check_dead_recursive(u);
        }
    }
}
impl StateGraph for SimpleStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        self.graph.ensure_edge(v1, v2);
    }
    fn mark_done_unchecked(&mut self, v: usize) {
        self.graph.overwrite_vertex(v, Status::Unknown);
        self.merge_all_cycles(v);
        self.check_dead_recursive(v);
    }
    fn get_status(&self, v: usize) -> Status {
        *self.graph.get_label_or_default(v)
    }
    fn vec_states(&self) -> Vec<usize> {
        self.graph.iter_vertices().collect()
    }
    fn get_space(&self) -> usize {
        self.graph.get_space()
    }
    fn get_time(&self) -> usize {
        self.graph.get_time() + self.additional_time.get()
    }
}
