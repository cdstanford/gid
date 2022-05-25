/*
    Euler Forest

    Implementation of log(n) graph connectivity for forests, i.e.
    undirected graphs that are distjoint unions of trees.

    SEE ALSO (heavily relies on): AvlForest in avl_forest.rs

    The data structure maintains a forest graph, and supports
    the following in O(log n) per operation:
    - Adding a new 1-vertex tree
    - Checking whether two vertices are in the same tree
    - Joining two trees into one by adding an edge
    - Splitting a tree into two by removing an edge

    Originally we tried to use Frederickson's Topology Trees,
    but they seem difficult to get right in the implementation.
    Now we are using Henzinger and King's Euler tour trees,
    implemented using a forest of balanced AVL trees.
    We find this implementation much nicer.

    This doesn't solve the problem of undirected connectivity in *general*
    graphs, but forests are enough for our use case.

    References:
    - Dynamic graph algorithms.
      David Eppstein, Zvi Galil, and Guiseppe Italiano.
      Algorithms and theory of computation handbook, 1999.
      (Good introduction and overview of topology trees)
    - Data structures for on-line updating of minimum spanning trees,
      with applications.
      Greg Frederickson.
      SIAM Journal on Computing, 1985.
      (Original definition of topology trees)
    -  Randomized Fully Dynamic Graph Algorithms with Polylogarithmic Time
       per Operation.
       Monika R. Henzinger and Valerie King.
       JACM, 1999.
     - Useful notes:
       http://courses.csail.mit.edu/6.851/spring07/scribe/lec05.pdf
*/

use std::fmt::Debug;
use std::hash::Hash;

use super::avl_forest::{AvlForest, IdType};

// Type used to identify nodes -- can be replaced with anything that
// identifies an edge or vertex uniquely:
// - An edge is represented as (u, v)
// - A vertex v is represented as (v, v)
#[derive(Copy, Debug, Eq, Hash, Ord, Clone, PartialEq, PartialOrd)]
struct NodeId<V: IdType>(V, V);
impl<V: IdType> NodeId<V> {
    fn edge(u: V, v: V) -> Self {
        debug_assert!(u != v);
        NodeId(u, v)
    }
    fn vert(v: V) -> Self {
        NodeId(v, v)
    }
}

/*
    The publicly exposed data structure
*/
#[derive(Debug)]
pub struct EulerForest<V: IdType> {
    nodes: AvlForest<NodeId<V>>,
}
impl<V: IdType> Default for EulerForest<V> {
    fn default() -> Self {
        Self { nodes: Default::default() }
    }
}
impl<V: IdType> EulerForest<V> {
    pub fn new() -> Self {
        let result: Self = Default::default();
        result.assert_invariant();
        result
    }
    pub fn ensure_vertex(&mut self, v: V) {
        self.nodes.ensure(NodeId::vert(v));
        self.assert_invariant();
    }
    pub fn add_edge(&mut self, v1: V, v2: V) {
        debug_assert!(self.is_seen(v1));
        debug_assert!(self.is_seen(v2));
        debug_assert!(!self.same_root(v1, v2));

        let e12 = NodeId::edge(v1, v2);
        let e21 = NodeId::edge(v2, v1);
        let v1 = NodeId::vert(v1);
        let v2 = NodeId::vert(v2);
        self.nodes.ensure(e12);
        self.nodes.ensure(e21);

        // Split trees at v1 and v2, saving neighbors...
        let u1 = self.nodes.prev(v1);
        let w1 = self.nodes.next(v1);
        let u2 = self.nodes.prev(v2);
        let w2 = self.nodes.next(v2);
        self.nodes.split(v1);
        self.nodes.split(v2);

        // Then piece the trees back together in order of a new Euler tour
        let r = self.nodes.get_root(v1);
        for node in [Some(e12), Some(v2), w2, u2, Some(e21), w1, u1] {
            if let Some(node) = node {
                self.nodes.concat(r, node);
            }
        }

        self.assert_invariant();
    }
    pub fn remove_edge(&mut self, v1: V, v2: V) {
        debug_assert!(self.is_seen(v1));
        debug_assert!(self.is_seen(v2));
        debug_assert!(self.same_root(v1, v2));
        let e12 = NodeId::edge(v1, v2);
        let e21 = NodeId::edge(v2, v1);

        // Neighbors
        let u1 = self.nodes.prev(e12);
        let u2 = self.nodes.next(e12);
        let u3 = self.nodes.prev(e21);
        let u4 = self.nodes.next(e21);

        // This splits into potentially 3 trees
        self.nodes.split(e12);
        self.nodes.split(e21);

        // Piece back together the first and last tree, if necessary
        // One of these should return true and the other false
        if let Some((u2, u3)) = u2.zip(u3) {
            self.nodes.concat(u2, u3);
        }
        if let Some((u4, u1)) = u4.zip(u1) {
            self.nodes.concat(u4, u1);
        }

        self.assert_invariant();
    }
    pub fn same_root(&self, v1: V, v2: V) -> bool {
        self.nodes.same_root(NodeId::vert(v1), NodeId::vert(v2))
    }

    /*
        Internal
    */
    fn is_seen(&self, v: V) -> bool {
        self.nodes.is_seen(NodeId::vert(v))
    }

    /*
        Invariant check
        TODO
    */
    #[cfg(debug_assertions)]
    fn assert_invariant(&self) {}
    #[cfg(not(debug_assertions))]
    fn assert_invariant(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vertex() {
        let mut g = EulerForest::new();
        assert!(!g.is_seen(1));
        assert!(!g.is_seen(2));
        assert!(!g.is_seen(3));
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.ensure_vertex(3);
        assert!(g.is_seen(1));
        assert!(g.is_seen(2));
        assert!(g.is_seen(1));
        assert!(!g.is_seen(0));
        assert!(!g.is_seen(4));

        assert!(g.same_root(2, 2));
        assert!(!g.same_root(1, 2));
        assert!(!g.same_root(2, 3));
        assert!(!g.same_root(3, 1));
    }

    #[test]
    #[should_panic]
    fn test_query_nonexistent_1() {
        let g = EulerForest::new();
        g.same_root(1, 1);
    }

    #[test]
    #[should_panic]
    fn test_query_nonexistent_2() {
        let g = EulerForest::new();
        g.same_root(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_edge_nonexistent_1() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.add_edge(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_edge_nonexistent_2() {
        let mut g = EulerForest::new();
        g.ensure_vertex(2);
        g.add_edge(1, 2);
    }

    #[test]
    fn test_two_vertices() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.add_edge(1, 2);
        assert!(g.same_root(1, 2));
        assert!(g.same_root(2, 1));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_add_edge_twice() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.add_edge(1, 2);
        g.add_edge(1, 2);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_add_self_edge() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.add_edge(1, 1);
    }

    #[test]
    fn test_add_edges() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.ensure_vertex(3);
        g.add_edge(1, 2);
        assert!(g.same_root(1, 2));
        assert!(!g.same_root(1, 3));
        assert!(!g.same_root(2, 3));
        g.add_edge(3, 2);
        assert!(g.same_root(1, 2));
        assert!(g.same_root(2, 3));
    }

    #[test]
    fn test_add_edges_complicated() {
        let mut g = EulerForest::new();
        for i in 0..10 {
            g.ensure_vertex(i);
        }
        g.add_edge(0, 1);
        g.add_edge(2, 3);
        g.add_edge(1, 3);
        g.add_edge(6, 5);
        g.add_edge(5, 4);
        g.add_edge(4, 7);
        g.add_edge(3, 8);
        g.add_edge(9, 2);

        assert!(g.same_root(0, 1));
        assert!(g.same_root(1, 2));
        assert!(g.same_root(2, 3));
        assert!(g.same_root(3, 8));
        assert!(g.same_root(8, 9));

        assert!(g.same_root(4, 5));
        assert!(g.same_root(5, 6));
        assert!(g.same_root(6, 7));

        assert!(!g.same_root(3, 4));
        assert!(!g.same_root(7, 8));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_add_cycle_2() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.add_edge(1, 2);
        g.add_edge(2, 1);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_add_cycle_3() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.ensure_vertex(3);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 1);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn test_add_cycle_4() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.ensure_vertex(3);
        g.ensure_vertex(4);
        g.add_edge(1, 2);
        g.add_edge(3, 4);
        g.add_edge(2, 3);
        g.add_edge(4, 1);
    }

    #[test]
    fn test_add_two_parents() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.ensure_vertex(3);
        g.add_edge(3, 1);
        assert!(g.same_root(1, 3));
        assert!(!g.same_root(1, 2));
        g.add_edge(3, 2);
        assert!(g.same_root(1, 2));
        assert!(g.same_root(2, 3));
    }

    #[test]
    fn test_remove_edge_1() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.ensure_vertex(3);
        g.ensure_vertex(4);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 4);
        assert!(g.same_root(1, 2));
        assert!(g.same_root(2, 3));
        assert!(g.same_root(3, 4));
        g.remove_edge(2, 3);
        assert!(g.same_root(1, 2));
        assert!(g.same_root(3, 4));
        assert!(!g.same_root(2, 3));
    }

    #[test]
    fn test_remove_edge_2() {
        let mut g = EulerForest::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.ensure_vertex(3);
        g.ensure_vertex(4);
        g.add_edge(3, 4);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(g.same_root(1, 4));
        g.remove_edge(1, 2);
        assert!(!g.same_root(1, 2));
        assert!(g.same_root(2, 3));
        assert!(g.same_root(3, 4));
        g.remove_edge(2, 3);
        assert!(!g.same_root(1, 2));
        assert!(!g.same_root(1, 3));
        assert!(!g.same_root(2, 3));
        assert!(g.same_root(3, 4));
    }
}
