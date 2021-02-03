/*
    The first and simplest naive implemenation implementing the
    state graph interface.

    This just stores the graph using hash tables, and
    does naive DFS to determine whether states are dead.
*/

use crate::graph::DiGraph;
use crate::interface::{StateGraph, Status};
use std::collections::HashSet;
use std::iter;

#[derive(Debug, Default)]
pub struct NaiveStateGraph {
    graph: DiGraph<usize, Status>,
}
impl NaiveStateGraph {
    fn calculate_new_live_states(&mut self, v: usize) {
        // Mark all states Live backwards from v, but not including v
        if self.is_live(v) {
            let new_live: HashSet<usize> = self
                .graph
                .dfs_bck(iter::once(v), |u| !self.is_live(u))
                .collect();
            for &u in new_live.iter() {
                self.graph.overwrite_vertex(u, Status::Live);
            }
        }
    }
    fn recalculate_dead_states(&mut self) {
        // Recalculate the subset of closed states that are dead: states
        // that can't reach an Open state (i.e. all reachable states are
        // dead or unknown).
        // This is the only nontrivial aspect of the naive implementation,
        // uses a DFS, and is worst-case O(m).

        // Initialize
        let (unkdead, openlive): (HashSet<usize>, HashSet<usize>) = self
            .graph
            .iter_vertices()
            .partition(|&v| self.is_unknown(v) || self.is_dead(v));
        let not_dead: HashSet<usize> = self
            .graph
            .dfs_bck(openlive.iter().copied(), |v| unkdead.contains(&v))
            .collect();

        // Mark not-not-dead states as dead
        for &v in unkdead.iter() {
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
        self.calculate_new_live_states(v2);
    }
    fn mark_closed_unchecked(&mut self, v: usize) {
        self.graph.overwrite_vertex(v, Status::Unknown);
        self.recalculate_dead_states();
    }
    fn mark_live_unchecked(&mut self, v: usize) {
        self.graph.overwrite_vertex(v, Status::Live);
        self.calculate_new_live_states(v);
    }
    fn not_reachable_unchecked(&mut self, _v1: usize, _v2: usize) {
        // Ignore NotReachable
    }
    fn get_status(&self, v: usize) -> Option<Status> {
        self.graph.get_label(v).copied()
    }
    fn get_space(&self) -> usize {
        self.graph.get_space()
    }
    fn get_time(&self) -> usize {
        self.graph.get_time()
    }
}
