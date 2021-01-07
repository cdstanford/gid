/*
    Implementation of the StateGraph trait using Tarjan's
    algorithm for online strongly-connected-components (the state of the art
    in terms of computational complexity).

    The algorithm we implement is described in section 4.1 of this paper:
        Bender, M. A., Fineman, J. T., Gilbert, S., & Tarjan, R. E. (2015).
        A new approach to incremental cycle detection and related problems.
        CM Transactions on Algorithms (TALG), 12(2), 1-22.

    As with all the implementations, we rely as much as possible on the core
    graph functionality in graph::DiGraph.
*/

use super::graph::DiGraph;
use super::interface::{StateGraph, Status};
use std::collections::HashSet;
use std::iter;

// The key to the algorithm: pseudo-topological numbering
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Level(usize);
impl Default for Level {
    fn default() -> Self {
        Level(0)
    }
}

#[derive(Debug, Default)]
pub struct TarjanStateGraph {
    graph: DiGraph<usize, (Status, Level)>,
}
impl TarjanStateGraph {
    fn set_status(&mut self, v: usize, status: Status) {
        debug_assert!(self.graph.is_seen(v));
        self.graph.get_label_mut(v).unwrap().0 = status;
    }
    fn _set_level(&mut self, v: usize, level: Level) {
        debug_assert!(self.graph.is_seen(v));
        self.graph.get_label_mut(v).unwrap().1 = level;
    }
    fn update_levels_iterative(&mut self, _v1: usize, _v2: usize) {
        // Update numbering after adding an edge (v1, v2),
        // AND ensure acyclic by merging cycles.
        // This is the main algorithm, as described in the Tarjan paper.
        // TODO
    }
    fn check_dead_iterative(&mut self, v: usize) {
        // This is the same procedure as in Simple
        let now_dead: HashSet<usize> = self
            .graph
            .topo_search_bck(
                iter::once(v),
                |u| !self.is_done(u),
                |w| self.is_dead(w),
            )
            .collect();
        debug_assert!(now_dead.is_empty() || now_dead.contains(&v));
        for &u in now_dead.iter() {
            self.set_status(u, Status::Dead);
        }
    }
}
impl StateGraph for TarjanStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        self.graph.ensure_edge(v1, v2);
        self.update_levels_iterative(v1, v2);
    }
    fn mark_done_unchecked(&mut self, v: usize) {
        self.set_status(v, Status::Unknown);
        self.check_dead_iterative(v);
    }
    fn get_status(&self, v: usize) -> Status {
        self.graph.get_label_or_default(v).0
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
