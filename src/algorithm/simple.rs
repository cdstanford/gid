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

use crate::graph::DiGraph;
use crate::interface::{StateGraph, Status};
use crate::util::FreshClone;
use std::collections::HashSet;
use std::iter;

#[derive(Debug, Default)]
pub struct SimpleStateGraph {
    graph: DiGraph<usize, Status>,
}
impl SimpleStateGraph {
    fn merge_vertices(&mut self, v1: usize, v2: usize) {
        // println!("  Merging: {} {}", v1, v2);
        debug_assert!(self.is_u_or_d(v1));
        debug_assert!(self.is_u_or_d(v2));
        debug_assert!(v1 != v2);
        self.graph.merge(v1, v2);
    }
    fn merge_all_cycles(&mut self, v: usize) {
        // println!("  Merging cycles through: {}", v);
        // Merge all cycles through v
        // (assuming no other cycles in closed states)
        debug_assert!(self.is_u_or_d(v));
        let fwd_reachable: HashSet<usize> =
            self.graph.dfs_fwd(iter::once(v), |w| self.is_u_or_d(w)).collect();
        for u in self
            .graph
            .dfs_bck(iter::once(v), |u| fwd_reachable.contains(&u))
            .fresh_clone()
        {
            // println!("  Found bireachable: {}", u);
            debug_assert!(u != v);
            self.merge_vertices(u, v);
        }
    }
    fn check_dead_iterative(&mut self, v: usize) {
        // Check if v is dead and recurse on back edges.
        // println!("  Checking if dead iteratively from: {}", v);
        for u in self
            .graph
            .topo_search_bck(v, |u| self.is_u_or_d(u), |w| !self.is_dead(w))
            .fresh_clone()
        {
            // println!("  Marking dead: {}", u);
            self.graph.overwrite_vertex(u, Status::Dead);
        }
    }
    fn calculate_new_live_states(&mut self, v: usize) {
        // Same fn as in Naive
        if self.is_live(v) {
            for u in self
                .graph
                .dfs_bck(iter::once(v), |u| !self.is_live_bck(u))
                .fresh_clone()
            {
                self.graph.overwrite_vertex(u, Status::Live);
            }
        }
    }
}
impl StateGraph for SimpleStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        // println!("Adding transition: {} {}", v1, v2);
        self.graph.ensure_edge(v1, v2);
        self.calculate_new_live_states(v2);
    }
    fn mark_closed_unchecked(&mut self, v: usize) {
        // println!("Marking closed: {}", v);
        self.graph.overwrite_vertex(v, Status::Unknown);
        self.merge_all_cycles(v);
        self.check_dead_iterative(v);
    }
    fn mark_live_unchecked(&mut self, v: usize) {
        // println!("Marking live: {}", v);
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
