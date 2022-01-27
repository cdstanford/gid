/*
    Implementation that uses topology trees (topology_tree.rs)
    to track which states are in the same component of the forest.
*/

use crate::graph::DiGraph;
use crate::interface::{StateGraph, Status};
use crate::topology_tree::TopTrees;
use std::collections::{HashSet, LinkedList};
use std::iter;

#[derive(Debug, Default, PartialEq)]
struct Node {
    // Reserve list: forward edges not added to graph.
    reserve: LinkedList<usize>,

    // Successor
    // Stored as an edge, rather than just a vertex,
    // to preserve the original ID in case of vertex merging.
    next: Option<(usize, usize)>,

    // Categorized status, same as in other algorithms
    status: Status,
}
fn merge_nodes(mut n1: Node, mut n2: Node) -> Node {
    // Note: result will be Status::Open!
    let mut result: Node = Default::default();
    debug_assert!(n1.status == Status::Unknown || n1.status == Status::Open);
    debug_assert!(n2.status == Status::Unknown || n2.status == Status::Open);
    debug_assert_eq!(result.status, Status::Open);
    result.reserve.append(&mut n1.reserve);
    result.reserve.append(&mut n2.reserve);
    result
}

#[derive(Debug, Default)]
pub struct SmartStateGraph {
    graph: DiGraph<usize, Node>,
    top_trees: TopTrees<usize>,
    // TODO: track time, if wanted for debug step counting
    // additional_time: DebugCounter,
}
impl SmartStateGraph {
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
        // Mark live in particular deletes reserve edges.
        if status == Status::Live {
            self.get_node_mut(v).reserve.clear();
        }
    }
    // Reserve edges getters / setters
    fn push_reserve(&mut self, v: usize, w: usize) {
        debug_assert!(self.is_seen(v));
        debug_assert!(!self.is_closed(v));
        self.get_node_mut(v).reserve.push_back(w);
    }
    fn pop_reserve(&mut self, v: usize) -> Option<usize> {
        debug_assert!(self.is_seen(v));
        debug_assert!(!self.is_closed(v));
        self.get_node_mut(v).reserve.pop_back()
    }
    // In this implementation, every vertex has at most one successor.
    fn get_succ(&self, v: usize) -> Option<usize> {
        debug_assert!(self.is_closed(v));
        self.get_node(v).next.map(|(_, w)| w)
    }
    fn set_succ(&mut self, v: usize, w: usize) {
        debug_assert_eq!(self.get_succ(v), None);
        self.get_node_mut(v).next = Some((v, w));
    }
    // Clear the node's successor and return the edge
    fn clear_succ(&mut self, v: usize) -> (usize, usize) {
        let mut result = None;
        std::mem::swap(&mut result, &mut self.get_node_mut(v).next);
        result.unwrap_or_else(|| {
            panic!("Called clear_succ on node without a successor");
        })
    }

    /*
        is-root
        Compare with the implementation in jump.rs
        In this implementation, we critically rely
        on topology trees for the efficient check. (TODO)

        Currently: do something naive and inefficient
    */
    fn is_root(&mut self, v: usize, end: usize) -> bool {
        debug_assert!(self.is_unknown(v) || self.is_open(v));
        debug_assert!(self.is_open(end));
        self.top_trees.same_root(v, end)
        // The following naive implementation works too, not using top_trees
        // if self.is_open(v) {
        //     self.graph.is_same_vertex(v, end)
        // } else {
        //     self.is_root(self.get_succ(v).unwrap(), end)
        // }
    }

    /*
        Merge the path from vertex v to the Open vertex it currently points
        to.
    */
    fn merge_path_from(&mut self, v: usize) {
        let to_merge: Vec<usize> = {
            iter::successors(Some(v), |&w| {
                // println!("{} {:?}", w, self.get_status(w));
                if self.is_closed(w) {
                    Some(self.get_succ(w).unwrap())
                } else {
                    None
                }
            })
            .collect()
        };
        for &w in &to_merge {
            // println!("  Merging: {}, {}", v, w);
            self.graph.merge_using(v, w, merge_nodes);
        }
    }

    /*
        Initialize function for a newly closed vertex, to find an univisted
        vertex.
    */
    fn is_succ(&self, u: usize, v: usize) -> bool {
        match self.get_succ(u) {
            Some(w) => self.graph.is_same_vertex(w, v),
            None => false,
        }
    }
    fn check_dead(&mut self, v: usize) {
        debug_assert!(self.is_open(v));
        while let Some(w) = self.pop_reserve(v) {
            if self.is_dead(w) {
                continue;
            } else if self.is_root(w, v) {
                // Merge cycle and continue
                // println!("  (merging {} -> {} -> ... -> {})", v, w, w_end);
                self.merge_path_from(w);
            } else {
                // No further work, set successor and return
                // println!("  (setting jump and returning)");
                self.set_status(v, Status::Unknown);
                self.set_succ(v, w);
                self.top_trees.add_edge(v, w);
                return;
            }
        }
        // No more edges -- v is dead.
        // println!("Found Dead: {}", v);
        // Recurse on all edges backwards from v.
        let to_recurse: HashSet<usize> = self
            .graph
            .iter_bck_edges(v)
            .filter(|&u| self.is_unknown(u))
            .filter(|&u| self.is_succ(u, v))
            .collect();
        // First set to dead
        self.set_status(v, Status::Dead);
        // Second set to_recurse as open so that recursive calls won't mess
        // with them
        for &u in &to_recurse {
            self.set_status(u, Status::Open);
            let (orig_u, orig_v) = self.clear_succ(u);
            // TODO: we might need to know u, v are canonical here. Do we?
            self.top_trees.remove_edge(orig_u, orig_v);
        }
        // Then go through and check dead for each one
        for &u in &to_recurse {
            // println!("  Recursing on: {}", u);
            self.check_dead(u);
        }
    }

    /*
        Calculate new live states
    */
    fn calculate_new_live_states(&mut self, v: usize) {
        // Same fn as in Naive
        if self.is_live(v) {
            let new_live: HashSet<usize> = self
                .graph
                .dfs_bck(iter::once(v), |u| {
                    debug_assert!(!self.is_dead(u));
                    !self.is_live(u)
                })
                .collect();
            for &u in new_live.iter() {
                self.set_status(u, Status::Live);
            }
        }
    }
}
impl StateGraph for SmartStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        // println!("# Adding transition: {}, {}", v1, v2);
        self.graph.ensure_edge_bck(v1, v2);
        self.top_trees.ensure_vertex(v1);
        self.top_trees.ensure_vertex(v2);
        self.calculate_new_live_states(v2);
        if !self.is_live(v1) {
            self.push_reserve(v1, v2);
        }
    }
    fn mark_closed_unchecked(&mut self, v: usize) {
        // println!("# Marking closed: {}", v);
        self.graph.ensure_vertex(v);
        self.top_trees.ensure_vertex(v);
        self.check_dead(v);
    }
    fn mark_live_unchecked(&mut self, v: usize) {
        // println!("# Marking live: {}", v);
        self.graph.ensure_vertex(v);
        self.top_trees.ensure_vertex(v);
        self.set_status(v, Status::Live);
        self.calculate_new_live_states(v);
    }
    fn not_reachable_unchecked(&mut self, _v1: usize, _v2: usize) {
        // Ignore NotReachable
    }
    fn get_status(&self, v: usize) -> Option<Status> {
        self.graph.get_label(v).map(|l| l.status)
    }
    fn get_space(&self) -> usize {
        self.graph.get_space()
    }
    fn get_time(&self) -> usize {
        self.graph.get_time()
    }
}
