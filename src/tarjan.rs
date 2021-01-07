/*
    Implementation of the StateGraph trait using Tarjan's
    algorithm for online strongly-connected-components (the state of the art
    in terms of computational complexity).

    As with all the implementations, we rely as much as possible on the core
    graph functionality in graph::DiGraph.
*/

use super::graph::DiGraph;
use super::interface::{StateGraph, Status};

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
    fn update_numbering(&mut self, _v1: usize, _v2: usize) {
        // Update numbering after adding an edge (v1, v2),
        // AND ensure acyclic by merging cycles.
        // TODO
    }
    fn mark_dead_recursive(&mut self, _v: usize) {
        // TODO: Basically the same procedure as in Simple
    }
}
impl StateGraph for TarjanStateGraph {
    fn new() -> Self {
        Default::default()
    }
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize) {
        self.graph.ensure_edge(v1, v2);
        self.update_numbering(v1, v2);
    }
    fn mark_done_unchecked(&mut self, v: usize) {
        let level = self.graph.get_label_or_default(v).1;
        self.graph.overwrite_vertex(v, (Status::Unknown, level));
        self.mark_dead_recursive(v);
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
