/*
    Slightly less naive implemenation implementing the
    state graph interface, this time using Union Find.

    Parallels state_graph.cpp in the Z3 code base -- found in
        z3/src/util/state_graph.h
    with one improvement: we used LinkedList instead of HashSet for
    storing edges, because it allows merging edge sets in O(1).
*/

use super::debug_counter::DebugCounter;
use super::interface::{StateGraph, Status};
use partitions::PartitionVec;
use std::collections::{HashMap, HashSet, LinkedList};

#[derive(Debug, Default)]
pub struct SimpleStateGraph {
    // Same as Naive
    seen: HashSet<usize>,
    done: HashSet<usize>,
    dead: HashSet<usize>,
    // Use LinkedList instead of HashMap for edges (O(1) list concatenation)
    fwd_edges: HashMap<usize, LinkedList<usize>>,
    bck_edges: HashMap<usize, LinkedList<usize>>,
    // Union-Find data structure for SCCs
    sccs: PartitionVec<usize>,
    // Debug mode statistics
    space: DebugCounter,
    time: DebugCounter,
}
impl SimpleStateGraph {
    fn ensure_seen(&mut self, v: usize) {
        if !self.seen.contains(&v) {
            self.seen.insert(v);
            debug_assert!(!self.fwd_edges.contains_key(&v));
            debug_assert!(!self.bck_edges.contains_key(&v));
            self.fwd_edges.insert(v, LinkedList::new());
            self.bck_edges.insert(v, LinkedList::new());
            self.time.inc();
            self.space.inc();
        }
    }
    fn recalculate_dead_states(&mut self) {
        // Initialize
        let mut to_visit = Vec::new();
        let mut not_dead = HashSet::new();
        for &v in &self.seen {
            if !self.done.contains(&v) {
                to_visit.push(v);
                not_dead.insert(v);
                self.time.inc();
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
                self.time.inc();
            }
            self.time.inc();
        }

        // Mark not-not-dead states as dead
        for &v in &self.done {
            debug_assert!(!(self.dead.contains(&v) && not_dead.contains(&v)));
            if !not_dead.contains(&v) {
                self.dead.insert(v);
            }
            self.time.inc();
        }
    }
}
impl StateGraph for SimpleStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        debug_assert!(!self.done.contains(&v1));
        debug_assert!(!self.dead.contains(&v1));
        self.ensure_seen(v1);
        self.ensure_seen(v2);
        self.fwd_edges.get_mut(&v1).unwrap().push_back(v2);
        self.bck_edges.get_mut(&v2).unwrap().push_back(v1);
        self.space.inc();
        self.time.inc();
    }
    fn mark_done_unchecked(&mut self, v: usize) {
        debug_assert!(!self.done.contains(&v));
        debug_assert!(!self.dead.contains(&v));
        self.done.insert(v);
        self.recalculate_dead_states();
        self.time.inc();
    }

    // Remaining boilerplate
    // See naive.rs for documentation, these are the same as there.
    fn get_status(&self, v: usize) -> Status {
        self.time.inc();
        if self.dead.contains(&v) {
            debug_assert!(self.done.contains(&v));
            Status::Dead
        } else if self.done.contains(&v) {
            Status::Unknown
        } else {
            Status::Unvisited
        }
    }
    fn vec_states(&self) -> Vec<usize> {
        self.seen.iter().copied().collect()
    }
    fn get_space(&self) -> usize {
        self.space.get()
    }
    fn get_time(&self) -> usize {
        self.time.get()
    }
}
