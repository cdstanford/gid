/*
    The first and simplest naive implemenation implementing the
    state graph interface.

    This just stores the graph using hash tables, and
    does naive DFS to determine whether states are dead.
*/

use super::debug_counter::DebugCounter;
use super::graph::DiGraph;
use super::interface::{StateGraph, Status};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct NaiveStateGraph {
    graph: DiGraph<usize, Status>,
    // Debug mode statistics -- on top of those tracked by graph
    additional_time: DebugCounter,
}
impl NaiveStateGraph {
    fn ensure_seen(&mut self, v: usize) {
        if !self.graph.is_seen(v) {
            self.graph.add_vertex(v, Status::Unvisited);
        }
    }
    fn set_status(&mut self, v: usize, st: Status) {
        self.graph.add_vertex(v, st);
    }
    fn is_done(&self, v: usize) -> bool {
        self.get_status(v) != Status::Unvisited
    }
    fn is_dead(&self, v: usize) -> bool {
        self.get_status(v) == Status::Dead
    }
    fn recalculate_dead_states(&mut self) {
        // Recalculate the subset of done states that are dead: states
        // that can't reach an Unvisited state (i.e. all reachable states are
        // dead or done).
        // This is the only nontrivial aspect of the naive implementation,
        // uses a DFS, and is worst-case O(m).

        // Initialize
        let mut to_visit = Vec::new();
        let mut done = HashSet::new();
        let mut not_dead = HashSet::new();
        for v in self.graph.iter_vertices() {
            if self.is_done(v) {
                done.insert(v);
            } else {
                to_visit.push(v);
                not_dead.insert(v);
            }
        }

        // DFS
        while !to_visit.is_empty() {
            let v = to_visit.pop().unwrap();
            for u in self.graph.iter_bck_edges(v) {
                if !not_dead.contains(&u) {
                    to_visit.push(u);
                    not_dead.insert(u);
                }
            }
            self.additional_time.inc();
        }

        // Mark not-not-dead states as dead
        for &v in &done {
            debug_assert!(!(self.is_dead(v) && not_dead.contains(&v)));
            if !not_dead.contains(&v) {
                self.set_status(v, Status::Dead);
            }
            self.additional_time.inc();
        }
    }
}
impl StateGraph for NaiveStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        debug_assert_eq!(self.get_status(v1), Status::Unvisited);
        self.ensure_seen(v1);
        self.ensure_seen(v2);
        self.graph.add_edge(v1, v2);
    }
    fn mark_done_unchecked(&mut self, v: usize) {
        debug_assert_eq!(self.get_status(v), Status::Unvisited);
        self.set_status(v, Status::Unknown);
        self.recalculate_dead_states();
    }
    fn get_status(&self, v: usize) -> Status {
        self.additional_time.inc();
        *self.graph.get_label(v).unwrap_or(&Status::Unvisited)
    }

    fn vec_states(&self) -> Vec<usize> {
        // Warning: self.time not increased here
        self.graph.iter_vertices().collect()
    }

    // Statistics
    // These panic if not in debug mode.
    fn get_space(&self) -> usize {
        self.graph.get_space()
    }
    fn get_time(&self) -> usize {
        self.graph.get_time() + self.additional_time.get()
    }
}
