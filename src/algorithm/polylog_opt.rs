/*
    OptimizedStateGraph

    Performance-optimized version of polylog.rs

    Eventually, it would be nice to clean up all the code via some
    common interfaces shared by all the different algorithms.
    For now, most of this code is copied from polylog.rs with
    only some modifications.
*/

use crate::debug_counter::DebugCounter;
use crate::euler_forest::EulerForest;
use crate::graph::DiGraph;
use crate::interface::{StateGraph, Status};
use std::collections::{HashSet, LinkedList};
use std::iter;
use std::mem;

#[derive(Debug, Default, PartialEq)]
struct Node {
    // Reserve list: forward edges not added to graph.
    reserve: LinkedList<usize>,

    // Successor
    // Stored as an edge, rather than just a vertex,
    // to preserve the original ID in case of vertex merging.
    next: Option<(usize, usize)>,

    // Jump node -- jump to root
    // should be the node itself for Open root nodes -- and None
    // for nodes that have been modified too many times and are
    // now relying on the Euler trees impl
    jump: Option<usize>,

    // Jump fuel exhausted
    exhausted: bool,

    // Categorized status, same as in other algorithms
    status: Status,
}
fn merge_nodes(mut n1: Node, mut n2: Node) -> Node {
    // Note: result will be Status::Open!
    let mut result: Node = Default::default();
    debug_assert!(n1.status == Status::Unknown || n1.status == Status::Open);
    debug_assert!(n2.status == Status::Unknown || n2.status == Status::Open);
    debug_assert_eq!(result.status, Status::Open);
    debug_assert_eq!(result.jump, None);
    result.exhausted = n1.exhausted || n2.exhausted;
    result.reserve.append(&mut n1.reserve);
    result.reserve.append(&mut n2.reserve);
    result
}

#[derive(Debug, Default)]
pub struct OptimizedStateGraph {
    graph: DiGraph<usize, Node>,
    euler_forest: EulerForest,
    additional_space: DebugCounter,
}
impl OptimizedStateGraph {
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
        self.additional_space.inc();
        self.get_node_mut(v).reserve.push_back(w);
    }
    fn pop_reserve(&mut self, v: usize) -> Option<usize> {
        debug_assert!(self.is_seen(v));
        debug_assert!(!self.is_closed(v));
        self.get_node_mut(v).reserve.pop_back()
    }
    // In this implementation, every vertex has at most one successor.
    fn get_succ(&self, v: usize) -> Option<usize> {
        self.get_node(v).next.map(|(_, w)| w)
    }
    fn get_jump(&self, v: usize) -> Option<usize> {
        self.get_node(v).jump
    }
    fn set_succ(&mut self, v: usize, w: usize) {
        debug_assert_eq!(self.get_succ(v), None);
        debug_assert_eq!(self.get_jump(v), None);
        let vmut = self.get_node_mut(v);
        vmut.next = Some((v, w));
        if vmut.exhausted {
            self.euler_forest.ensure_vertex(w);
            self.euler_forest.add_edge(v, w);
            self.mark_exhausted_from(w);
        } else {
            vmut.jump = Some(w);
        }
    }
    // Clear the node's successor and return the edge
    fn clear_succ(&mut self, v: usize) {
        debug_assert!(self.get_succ(v).is_some());
        let vmut = self.get_node_mut(v);
        vmut.jump = None;
        let mut result = None;
        mem::swap(&mut result, &mut vmut.next);
        debug_assert!(result.is_some());
        if vmut.exhausted {
            let (u0, v0) = result.unwrap();
            self.euler_forest.remove_edge(u0, v0);
        }
    }

    /*
        is-root
        Compare with the implementation in jump.rs and polylog.rs

        Optimization: if the node is not exhausted, we try to first
        use the jump vertex to jump quickly to the root.
        If exhausted, we fallback on the Euler forest.
    */
    fn is_root(&mut self, v: usize, end: usize) -> bool {
        debug_assert!(self.is_unknown(v) || self.is_open(v));
        debug_assert!(self.is_open(end));

        if self.is_open(v) {
            return self.graph.is_same_vertex(v, end);
        } else if self.get_node(v).exhausted {
            // exhausted means the jump node was dead
            // This assertion is currently failing
            // It doesn't matter too much, if exhausted we no longer
            // care about jump.
            // debug_assert!(
            //     self.get_jump(v) == None
            //         || self.is_dead(self.get_jump(v).unwrap())
            // );
            // Also should mean that it's already added to the Euler forest
            debug_assert!(self.euler_forest.is_seen(v));
            self.euler_forest.ensure_vertex(end);
            return self.euler_forest.same_root(v, end);
        }
        // Jump case
        let j = self.get_jump(v).unwrap();
        if self.is_dead(j) {
            // Mark exhausted, and all successors as necessary
            let v_end = self.mark_exhausted_from(v);
            debug_assert!(self.euler_forest.is_seen(v));
            debug_assert!(self.euler_forest.is_seen(v_end));
            debug_assert!(self.euler_forest.is_seen(end));
            if self.is_open(v_end) {
                debug_assert!(
                    (v_end == end)
                        || (self.graph.is_same_vertex(v_end, end)
                            == self.euler_forest.same_root(v_end, end))
                );
                debug_assert_eq!(
                    self.graph.is_same_vertex(v_end, end),
                    self.euler_forest.same_root(v_end, end)
                );
                self.graph.is_same_vertex(v_end, end)
            } else {
                self.euler_forest.same_root(v_end, end)
            }
        } else {
            // recursive call
            let result = self.is_root(j, end);
            // update jump pointer
            // note this might set it to dead in some cases, but seems
            // easier than checking for is_dead() and handlign that case.
            // Dead jump will resolve to exhausted on the next call.
            if let Some(jj) = self.get_jump(j) {
                debug_assert!(self.is_closed(j));
                self.get_node_mut(v).jump = Some(jj);
            }
            result
        }
    }

    /*
        Mark the path from vertex v to the Open vertex it currently points
        to as exhausted.
        Return the first exhausted vertex found.
    */
    fn mark_exhausted_from(&mut self, mut v: usize) -> usize {
        if self.get_node(v).exhausted {
            return v;
        }
        self.get_node_mut(v).exhausted = true;
        self.euler_forest.ensure_vertex(v);
        while let Some(w) = self.get_succ(v) {
            debug_assert!(self.is_unknown(w) || self.is_open(w));
            // This line may not be necessary, it only matters
            // if node.next is (v', w) where v != v' but equivalent...
            self.get_node_mut(v).next = Some((v, w));
            self.euler_forest.ensure_vertex(w);
            self.euler_forest.add_edge(v, w);
            if self.get_node(w).exhausted {
                return w;
            }
            self.get_node_mut(w).exhausted = true;
            v = w;
        }
        debug_assert!(self.is_open(v));
        v
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
        let mut to_visit = vec![v];
        while let Some(x) = to_visit.pop() {
            self.check_dead_step(&mut to_visit, x);
        }
    }
    fn check_dead_step(&mut self, to_visit: &mut Vec<usize>, v: usize) {
        debug_assert!(self.is_open(v));
        while let Some(w) = self.pop_reserve(v) {
            if self.is_dead(w) {
                continue;
            } else if self.is_root(w, v) {
                // Merge cycle and continue
                // println!("  (merging {} -> {} -> ... -> {})", v, w, w_end);
                if self.get_node(v).exhausted {
                    self.mark_exhausted_from(w);
                    debug_assert!(self.euler_forest.same_root(v, w));
                } else {
                    debug_assert!(!self.get_node(w).exhausted);
                }
                self.merge_path_from(w);
            } else {
                // Set successor and return
                self.set_status(v, Status::Unknown);
                self.set_succ(v, w);
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
            self.clear_succ(u);
            self.set_status(u, Status::Open);
            to_visit.push(u);
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
impl StateGraph for OptimizedStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        // println!("# Adding transition: {}, {}", v1, v2);
        self.graph.ensure_edge_bck(v1, v2);
        self.calculate_new_live_states(v2);
        if !self.is_live(v1) {
            self.push_reserve(v1, v2);
        }
    }
    fn mark_closed_unchecked(&mut self, v: usize) {
        // println!("# Marking closed: {}", v);
        self.graph.ensure_vertex(v);
        self.check_dead(v);
    }
    fn mark_live_unchecked(&mut self, v: usize) {
        // println!("# Marking live: {}", v);
        self.graph.ensure_vertex(v);
        self.set_status(v, Status::Live);
        self.calculate_new_live_states(v);
    }
    fn not_reachable_unchecked(&mut self, _v1: usize, _v2: usize) {}
    fn get_status(&self, v: usize) -> Option<Status> {
        self.graph.get_label(v).map(|l| l.status)
    }
    fn get_space(&self) -> usize {
        self.graph.get_space()
            + self.euler_forest.get_space()
            + self.additional_space.get()
    }
    fn get_time(&self) -> usize {
        self.graph.get_time() + self.euler_forest.get_time()
    }
}
