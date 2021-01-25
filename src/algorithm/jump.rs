/*
    Our new implementation of the StateGraph trait.
    Uses "jump" pointers to jump from each state a large number
    of states ahead at once.
*/

use crate::debug_counter::DebugCounter;
use crate::graph::DiGraph;
use crate::interface::{StateGraph, Status};
use std::iter;

#[derive(Debug, Default, PartialEq)]
struct Node {
    // jump list: nonempty for closed vertices.
    // First is a real edge, and the ith is approximately 2^i edges forward.
    jumps: Vec<usize>,
    // reserve list: draining copy of fwd_edges. When depleted, node is dead.
    reserve: Vec<usize>,
    status: Status,
}

#[derive(Debug, Default)]
pub struct JumpStateGraph {
    graph: DiGraph<usize, Node>,
    additional_time: DebugCounter,
}
impl JumpStateGraph {
    /* Node label manipulation */
    fn get_node(&self, v: usize) -> &Node {
        debug_assert!(self.is_seen(v));
        self.graph.get_label(v).unwrap()
    }
    fn get_node_mut(&mut self, v: usize) -> &mut Node {
        debug_assert!(self.is_seen(v));
        self.graph.get_label_mut(v).unwrap()
    }
    // Status getters / setters
    fn set_status(&mut self, v: usize, status: Status) {
        // println!("  Set status: {} {:?}", v, status);
        debug_assert!(self.is_seen(v));
        self.get_node_mut(v).status = status;
    }
    // Reserve edges getters / setters
    fn push_reserve(&mut self, v: usize, w: usize) {
        debug_assert!(self.is_seen(v));
        debug_assert!(!self.is_closed(v));
        self.get_node_mut(v).reserve.push(w);
    }
    fn pop_reserve(&mut self, v: usize) -> Option<usize> {
        debug_assert!(!self.is_closed(v));
        self.get_node_mut(v).reserve.pop()
    }
    // Jump list getters / setters
    fn get_nth_jump(&self, v: usize, n: usize) -> usize {
        debug_assert!(self.is_closed(v));
        debug_assert!(self.get_node(v).jumps.len() > n);
        self.get_node(v).jumps[n]
    }
    fn get_first_jump(&self, v: usize) -> usize {
        // println!("get_first_jump: {} {}", v, self.is_closed(v));
        debug_assert!(self.is_closed(v));
        debug_assert!(!self.get_node(v).jumps.is_empty());
        self.get_nth_jump(v, 0)
    }
    fn get_last_jump(&self, v: usize) -> usize {
        // Get the current last element in the jumps list.
        debug_assert!(self.is_closed(v));
        debug_assert!(!self.get_node(v).jumps.is_empty());
        *self.get_node(v).jumps.last().unwrap()
    }
    fn get_num_jumps(&self, v: usize) -> usize {
        // Get the length of the jumps list
        // (open vertices implicitly have no jumps)
        if self.is_closed(v) {
            debug_assert!(!self.get_node(v).jumps.is_empty());
            self.get_node(v).jumps.len()
        } else {
            0
        }
    }
    fn pop_last_jump(&mut self, v: usize) {
        // Remove the current last element in the jumps list.
        // println!("  Popping last jump: {}", v);
        debug_assert!(self.is_closed(v));
        debug_assert!(!self.get_node(v).jumps.is_empty());
        self.get_node_mut(v).jumps.pop();
    }
    fn clear_jumps(&mut self, v: usize) {
        // println!("  Clearing jumps: {}", v);
        debug_assert!(self.is_closed(v));
        debug_assert!(!self.get_node(v).jumps.is_empty());
        self.get_node_mut(v).jumps.clear();
    }
    fn push_last_jump(&mut self, v: usize, w: usize) {
        // Add a last element to the jumps list.
        // println!("  Pushing jump: {}, {}", v, w);
        debug_assert!(self.is_closed(v));
        self.get_node_mut(v).jumps.push(w);
    }

    /*
        Main jump function

        Jump from v to the Univisted vertex it currently points to.
        The assumption / invariant is that although some jumps
        may be obsolete, there is always an open vertex that
        is pointed to once obsolete jumps are removed.
    */
    fn jump(&mut self, v: usize) -> usize {
        debug_assert!(self.graph.is_seen(v));
        if !self.is_closed(v) {
            return v;
        }
        // Pop dead jumps
        while self.is_dead(self.get_last_jump(v)) {
            self.pop_last_jump(v);
        }
        // Get result and update jumps list
        let w = self.get_last_jump(v);
        let result = self.jump(w);
        if self.get_num_jumps(v) <= self.get_num_jumps(w) {
            let new_jump = self.get_nth_jump(w, self.get_num_jumps(v) - 1);
            self.push_last_jump(v, new_jump);
        }
        result
    }

    /*
        Merge the path from vertex v to the Open vertex it currently points
        to.
    */
    fn merge_path_from(&mut self, v: usize) {
        let to_merge: Vec<usize> = {
            iter::successors(Some(v), |&w| {
                // println!("{} {:?} {}", w, self.get_status(w));
                if self.is_closed(w) {
                    Some(self.get_first_jump(w))
                } else {
                    None
                }
            })
            .collect()
        };
        for &w in &to_merge {
            // println!("  Merging: {}, {}", v, w);
            self.graph.merge_using(v, w, |_label1, _label2| Default::default());
        }
    }

    /*
        Initialize function for a newly closed vertex, to find an univisted
        vertex.
    */
    fn initialize_jumps(&mut self, v: usize) {
        // println!("Initializing jumps from: {}", v);
        while let Some(w) = self.pop_reserve(v) {
            if self.is_dead(w) {
                // println!("  (dead)");
                continue;
            }
            let w_end = self.jump(w);
            if self.graph.is_same_vertex(v, w_end) {
                // Merge cycle and continue
                // println!("  (merging {} -> {} -> ... -> {})", v, w, w_end);
                self.merge_path_from(w);
            } else {
                // No further work, set jump and return
                // println!("  (setting jump and returning)");
                debug_assert!(self.get_node(v).jumps.is_empty());
                self.set_status(v, Status::Unknown);
                self.push_last_jump(v, w);
                return;
            }
        }
        // No more edges -- v is dead.
        // Recurse on all edges backwards from v.
        self.set_status(v, Status::Dead);
        // println!("Found Dead: {}", v);
        let to_recurse: Vec<usize> = self
            .graph
            .iter_bck_edges(v)
            .filter(|&u| self.is_closed(u))
            .filter(|&u| self.graph.is_same_vertex(self.get_first_jump(u), v))
            .collect();
        // First set to_recurse as open so that recursive calls won't mess
        // with them
        for &u in &to_recurse {
            // println!("  Recursing on: {}", u);
            self.clear_jumps(u);
            self.set_status(u, Status::Open);
        }
        // Then go through and initialize jumps for each one
        for &u in &to_recurse {
            // println!("  Recursing on: {}", u);
            self.initialize_jumps(u);
        }
    }
}
impl StateGraph for JumpStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        // println!("# Adding transition: {}, {}", v1, v2);
        self.graph.ensure_edge(v1, v2);
        self.push_reserve(v1, v2);
    }
    fn mark_closed_unchecked(&mut self, v: usize) {
        // println!("# Marking Closed: {}", v);
        self.graph.ensure_vertex(v);
        self.initialize_jumps(v);
    }
    fn get_status(&self, v: usize) -> Option<Status> {
        self.graph.get_label(v).map(|l| l.status)
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
