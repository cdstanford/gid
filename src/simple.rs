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

use super::graph::DiGraph;
use super::interface::{StateGraph, Status};
use std::collections::HashSet;
use std::iter;

#[derive(Debug, Default)]
pub struct SimpleStateGraph {
    graph: DiGraph<usize, Status>,
}
impl SimpleStateGraph {
    fn merge_vertices(&mut self, v1: usize, v2: usize) {
        debug_assert!(self.is_done(v1));
        debug_assert!(self.is_done(v2));
        debug_assert!(v1 != v2);
        self.graph.merge(v1, v2);
    }
    fn merge_all_cycles(&mut self, v: usize) {
        // Merge all cycles through v (assuming no other cycles in Done states)
        debug_assert!(self.is_done(v));
        let fwd_reachable: HashSet<usize> =
            self.graph.dfs_fwd(iter::once(v), |w| !self.is_done(w)).collect();
        let bi_reachable: HashSet<usize> = self
            .graph
            .dfs_bck(iter::once(v), |u| !fwd_reachable.contains(&u))
            .collect();
        for &u in &bi_reachable {
            debug_assert!(u != v);
            self.merge_vertices(u, v);
        }
    }
    fn check_dead_recursive(&mut self, v: usize) {
        // Check if v is dead and recurse on back edges.
        // TODO: I think this implementation may be buggy
        // (failure case would be a diamond).
        // Replace with a topologically-sorting search function in search.rs
        // and graph.rs

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
        self.graph.iter_vertices_all().collect()
    }
    fn get_space(&self) -> usize {
        self.graph.get_space()
    }
    fn get_time(&self) -> usize {
        self.graph.get_time()
    }
}
