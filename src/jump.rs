/*
    Our new implementation of the StateGraph trait.
    Uses "jump" pointers to jump from each state a large number
    of states ahead at once.

    TODO (Planned)
*/

use super::debug_counter::DebugCounter;
use super::graph::DiGraph;
use super::interface::{StateGraph, Status};
use std::iter;

#[derive(Debug, Default, PartialEq)]
struct Node {
    jumps: Vec<usize>,
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

    /* Jump list getters / setters */
    fn get_nth_jump(&self, v: usize, n: usize) -> usize {
        debug_assert!(self.is_done(v));
        debug_assert!(self.graph.get_label(v).unwrap().jumps.len() > n);
        self.graph.get_label(v).unwrap().jumps[n]
    }
    fn get_first_jump(&self, v: usize) -> usize {
        self.get_nth_jump(v, 0)
    }
    fn get_last_jump(&self, v: usize) -> usize {
        // Get the current last element in the jumps list.
        debug_assert!(self.is_done(v));
        debug_assert!(!self.graph.get_label(v).unwrap().jumps.is_empty());
        *self.graph.get_label(v).unwrap().jumps.last().unwrap()
    }
    fn get_num_jumps(&self, v: usize) -> usize {
        // Get the length of the jumps list
        // (unvisited vertices implicitly have no jumps)
        if self.is_done(v) {
            debug_assert!(!self.graph.get_label(v).unwrap().jumps.is_empty());
            self.graph.get_label(v).unwrap().jumps.len()
        } else {
            0
        }
    }
    fn pop_last_jump(&mut self, v: usize) {
        // Remove the current last element in the jumps list.
        debug_assert!(self.is_done(v));
        debug_assert!(!self.graph.get_label(v).unwrap().jumps.is_empty());
        self.graph.get_label_mut(v).unwrap().jumps.pop();
    }
    fn clear_jumps(&mut self, v: usize) {
        debug_assert!(self.is_done(v));
        debug_assert!(!self.graph.get_label(v).unwrap().jumps.is_empty());
        self.graph.get_label_mut(v).unwrap().jumps.clear();
    }
    fn push_last_jump(&mut self, v: usize, w: usize) {
        // Add a last element to the jumps list.
        debug_assert!(self.is_done(v));
        self.graph.get_label_mut(v).unwrap().jumps.push(w);
    }

    /*
        Main jump function

        Jump from v to the Univisted vertex it currently points to.
        The assumption / invariant is that although some jumps
        may be obsolete, there is always an unvisited vertex that
        is pointed to once obsolete jumps are removed.
    */
    fn jump(&mut self, v: usize) -> usize {
        debug_assert!(self.graph.is_seen(v));
        if !self.is_done(v) {
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
        Merge the path from vertex v to the Unvisited vertex it currently points
        to.
    */
    fn merge_path_from(&mut self, v: usize) {
        let to_merge: Vec<usize> = {
            iter::successors(Some(v), |&w| {
                if self.is_done(w) {
                    Some(self.get_first_jump(v))
                } else {
                    None
                }
            })
            .collect()
        };
        for &w in &to_merge {
            self.graph.merge_using(v, w, |_label1, _label2| Default::default());
        }
    }

    /*
        Initialize function for a newly done vertex, to find an univisted
        vertex.
    */
    fn initialize_jumps(&mut self, v: usize) {
        while let Some(w) = self.graph.pop_edge_fwd(v) {
            if self.is_dead(w) {
                continue;
            }
            let w_end = self.jump(w);
            if self.graph.is_same_vertex(v, w_end) {
                // Merge cycle and continue
                self.merge_path_from(v);
            } else {
                // No further work, set jump and return
                debug_assert!(self
                    .graph
                    .get_label(v)
                    .unwrap()
                    .jumps
                    .is_empty());
                self.set_status(v, Status::Unknown);
                self.push_last_jump(v, w);
                return;
            }
        }
        // No more edges -- v is dead.
        // Recurse on all edges backwards from v.
        let to_recurse: Vec<usize> = self.graph.iter_bck_edges(v).collect();
        for &u in &to_recurse {
            if self.is_done(u)
                && self.graph.is_same_vertex(self.get_first_jump(u), v)
            {
                self.clear_jumps(u);
                self.initialize_jumps(u);
            }
        }
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
        self.initialize_jumps(v);
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
