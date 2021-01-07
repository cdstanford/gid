/*
    The first and simplest naive implemenation implementing the
    state graph interface.

    This just stores the graph using hash tables, and
    does naive DFS to determine whether states are dead.
*/

use super::graph::DiGraph;
use super::interface::{StateGraph, Status};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct NaiveStateGraph {
    graph: DiGraph<usize, Status>,
}
impl NaiveStateGraph {
    fn recalculate_dead_states(&mut self) {
        // Recalculate the subset of done states that are dead: states
        // that can't reach an Unvisited state (i.e. all reachable states are
        // dead or done).
        // This is the only nontrivial aspect of the naive implementation,
        // uses a DFS, and is worst-case O(m).

        // Initialize
        let (done, unvisited): (HashSet<usize>, HashSet<usize>) =
            self.graph.iter_vertices().partition(|&v| self.is_done(v));
        let not_dead: HashSet<usize> = self
            .graph
            .dfs_bck(unvisited.iter().copied(), |v| !done.contains(&v))
            .collect();

        // Mark not-not-dead states as dead
        for &v in done.iter() {
            debug_assert!(!(self.is_dead(v) && not_dead.contains(&v)));
            if !not_dead.contains(&v) {
                self.graph.overwrite_vertex(v, Status::Dead);
            }
        }
    }
}
impl StateGraph for NaiveStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        self.graph.ensure_edge(v1, v2);
    }
    fn mark_done_unchecked(&mut self, v: usize) {
        self.graph.overwrite_vertex(v, Status::Unknown);
        self.recalculate_dead_states();
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
        self.graph.get_time()
    }
}
