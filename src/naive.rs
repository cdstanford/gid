/*
    The first and simplest naive implemenation implementing the
    state graph interface.

    This just stores the graph using hash tables, and
    does naive DFS to determine whether states are dead.
*/

use super::interface::{StateGraph, Status};
use std::collections::{HashMap, HashSet};

struct NaiveStateGraph {
    seen: HashSet<usize>,
    done: HashSet<usize>,
    dead: HashSet<usize>,
    fwd_edges: HashMap<usize, HashSet<usize>>,
    bck_edges: HashMap<usize, HashSet<usize>>,
}
impl NaiveStateGraph {
    fn ensure_seen(&mut self, v: usize) {
        if !self.seen.contains(&v) {
            self.seen.insert(v);
            debug_assert!(!self.fwd_edges.contains_key(&v));
            debug_assert!(!self.bck_edges.contains_key(&v));
            self.fwd_edges.insert(v, HashSet::new());
            self.bck_edges.insert(v, HashSet::new());
        }
    }
    fn recalculate_dead_states(&mut self) {
        // Recalculate the subset of done states that are dead: states
        // that can't reach an Unvisited state (i.e. all reachable states are
        // dead or done).
        // This is the only nontrivial aspect of the naive implementation,
        // uses a DFS, and is worst-case O(m).

        // Initialize
        let mut to_visit = Vec::new();
        let mut not_dead = HashSet::new();
        for &v in &self.seen {
            if !self.done.contains(&v) {
                to_visit.push(v);
                not_dead.insert(v);
            }
        }

        // DFS
        while !to_visit.is_empty() {
            let v = to_visit.pop().unwrap();
            for &u in &self.bck_edges[&v] {
                if !not_dead.contains(&u) {
                    to_visit.push(u);
                    not_dead.insert(u);
                }
            }
        }

        // Mark not-not-dead states as dead
        for &v in &self.done {
            debug_assert!(!(self.dead.contains(&v) && not_dead.contains(&v)));
            if !not_dead.contains(&v) {
                self.dead.insert(v);
            }
        }
    }
}
impl StateGraph for NaiveStateGraph {
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        debug_assert!(!self.done.contains(&v1));
        debug_assert!(!self.dead.contains(&v1));
        self.ensure_seen(v1);
        self.ensure_seen(v2);
        self.fwd_edges.get_mut(&v1).unwrap().insert(v2);
        self.bck_edges.get_mut(&v2).unwrap().insert(v1);
    }
    fn mark_done_unchecked(&mut self, v: usize) {
        debug_assert!(!self.done.contains(&v));
        debug_assert!(!self.dead.contains(&v));
        self.done.insert(v);
        self.recalculate_dead_states();
    }
    fn get_status(&self, v: usize) -> Status {
        if self.dead.contains(&v) {
            debug_assert!(self.done.contains(&v));
            Status::Dead
        } else if self.done.contains(&v) {
            Status::Unknown
        } else {
            Status::Unvisited
        }
    }
}
