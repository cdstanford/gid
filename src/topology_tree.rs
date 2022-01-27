/*
    Implementation of Frederickson's Topology Trees.
    for dynamic undirected graph connectivity problems.

    The data structure maintains a forest of rooted trees, and supports
    the following in O(log n) per operation:
    - Adding a new 1-vertex tree
    - Querying the root vertex corresponding to a vertex
    - Setting a vertex as the root
    - Joining two trees into one by adding an edge
    - Splitting a tree into two by removing an edge

    Currently this is a placeholder. TODOs are
    1. implement naive version that works but isn't efficient
        ==> Done
    2. write unit tests
        ==> Done
    3. implement the efficient version

    For simplicity assumes vertices are `usize` values. Use
    a separate data structure to map values of another type
    uniquely to IDs.

    References:
    - Dynamic graph algorithms.
      David Eppstein, Zvi Galil, and Guiseppe Italiano.
      Algorithms and theory of computation handbook, 1999.
      (Good introduction and overview of topology trees)
    - Data structures for on-line updating of minimum spanning trees, with applications.
      Greg Frederickson.
      SIAM Journal on Computing, 1985.
      (Original definition of topology trees)
*/

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct TopTrees<V: Copy + Debug + Eq + Hash + Ord> {
    parents: HashMap<V, Option<V>>,
}
impl<V: Copy + Debug + Eq + Hash + Ord> Default for TopTrees<V> {
    fn default() -> Self {
        let parents = Default::default();
        Self { parents }
    }
}
impl<V: Copy + Debug + Eq + Hash + Ord> TopTrees<V> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add_vertex(&mut self, v: V) {
        assert!(!self.is_seen(v));
        self.parents.insert(v, None);
    }
    pub fn ensure_vertex(&mut self, v: V) {
        if !self.is_seen(v) {
            self.parents.insert(v, None);
        }
    }
    pub fn query_root(&self, mut v: V) -> V {
        assert!(self.is_seen(v));
        while !self.is_root(v) {
            v = self.get_parent(v).unwrap();
        }
        v
    }
    pub fn set_root(&mut self, v: V) {
        assert!(self.is_seen(v));
        // This is implemented recursively
        // Switches all edges from v to the root.
        if !self.is_root(v) {
            let next = self.get_parent(v).unwrap();
            self.set_root(next);
            debug_assert!(self.is_root(next));
            self.set_parent(next, v);
            self.erase_parent(v);
        }
        debug_assert!(self.is_root(v));
    }
    pub fn add_edge(&mut self, v1: V, v2: V) {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        assert!(!self.same_root(v1, v2));
        self.set_root(v1);
        debug_assert!(self.is_root(v1));
        self.set_parent(v1, v2);
    }
    pub fn remove_edge(&mut self, v1: V, v2: V) {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        // Set root -- to ensure edge goes in the expected direction
        self.set_root(v2);
        assert_eq!(self.get_parent(v1), Some(v2));
        debug_assert!(self.same_root(v1, v2));
        self.erase_parent(v1);
    }

    /*
        Derived
    */
    pub fn same_root(&self, v1: V, v2: V) -> bool {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        self.query_root(v1) == self.query_root(v2)
    }

    /*
        Internal
    */
    // Basic primitives
    fn is_seen(&self, v: V) -> bool {
        self.parents.contains_key(&v)
    }
    fn get_parent(&self, v: V) -> Option<V> {
        assert!(self.is_seen(v));
        *self.parents.get(&v).unwrap()
    }
    fn is_root(&self, v: V) -> bool {
        assert!(self.is_seen(v));
        self.get_parent(v).is_none()
    }
    // Parent map modifiers
    fn set_parent(&mut self, v1: V, v2: V) {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        *self.parents.get_mut(&v1).unwrap() = Some(v2);
    }
    fn erase_parent(&mut self, v1: V) {
        assert!(self.is_seen(v1));
        *self.parents.get_mut(&v1).unwrap() = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vertex() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        assert!(g.is_root(1));
        assert!(g.is_root(2));
        assert!(g.is_root(3));
    }

    #[test]
    #[should_panic]
    fn test_add_vertex_twice() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(1);
    }

    #[test]
    #[should_panic]
    fn test_query_nonexistent_1() {
        let g = TopTrees::new();
        g.is_root(1);
    }

    #[test]
    #[should_panic]
    fn test_query_nonexistent_2() {
        let g = TopTrees::new();
        g.query_root(1);
    }

    #[test]
    #[should_panic]
    fn test_edge_nonexistent_1() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_edge(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_edge_nonexistent_2() {
        let mut g = TopTrees::new();
        g.add_vertex(2);
        g.add_edge(1, 2);
    }

    #[test]
    fn test_two_vertices() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_edge(1, 2);
        assert_eq!(g.query_root(1), 2);
        assert_eq!(g.query_root(2), 2);
    }

    #[test]
    #[should_panic]
    fn test_add_edge_twice() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_edge(1, 2);
        g.add_edge(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_add_self_edge() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_edge(1, 1);
    }

    #[test]
    fn test_add_edges() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        g.add_edge(1, 2);
        assert_eq!(g.query_root(1), 2);
        assert_eq!(g.query_root(2), 2);
        assert_eq!(g.query_root(3), 3);
        g.add_edge(3, 2);
        assert_eq!(g.query_root(1), 2);
        assert_eq!(g.query_root(2), 2);
        assert_eq!(g.query_root(3), 2);
    }

    #[test]
    fn test_add_edges_complicated() {
        let mut g = TopTrees::new();
        for i in 0..10 {
            g.add_vertex(i);
        }
        g.add_edge(0, 1);
        g.add_edge(2, 3);
        g.add_edge(1, 3);
        g.add_edge(6, 5);
        g.add_edge(5, 4);
        g.add_edge(4, 7);
        g.add_edge(3, 8);
        g.add_edge(9, 2);
        assert_eq!(g.query_root(0), 8);
        assert_eq!(g.query_root(1), 8);
        assert_eq!(g.query_root(2), 8);
        assert_eq!(g.query_root(3), 8);
        assert_eq!(g.query_root(4), 7);
        assert_eq!(g.query_root(5), 7);
        assert_eq!(g.query_root(6), 7);
        assert_eq!(g.query_root(7), 7);
        assert_eq!(g.query_root(8), 8);
        assert_eq!(g.query_root(9), 8);
    }

    #[test]
    #[should_panic]
    fn test_add_cycle_2() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_edge(1, 2);
        g.add_edge(2, 1);
    }

    #[test]
    #[should_panic]
    fn test_add_cycle_3() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 1);
    }

    #[test]
    #[should_panic]
    fn test_add_cycle_4() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        g.add_vertex(4);
        g.add_edge(1, 2);
        g.add_edge(3, 4);
        g.add_edge(2, 3);
        g.add_edge(4, 1);
    }

    #[test]
    fn test_set_root_1() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_edge(1, 2);
        g.set_root(1);
        assert_eq!(g.query_root(1), 1);
        assert_eq!(g.query_root(2), 1);
    }

    #[test]
    fn test_set_root_2() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        g.add_vertex(4);
        g.add_edge(1, 2);
        g.add_edge(4, 3);
        g.add_edge(2, 3);
        assert_eq!(g.query_root(1), 3);
        assert_eq!(g.query_root(2), 3);
        assert_eq!(g.query_root(3), 3);
        assert_eq!(g.query_root(4), 3);
        g.set_root(1);
        assert_eq!(g.query_root(1), 1);
        assert_eq!(g.query_root(2), 1);
        assert_eq!(g.query_root(3), 1);
        assert_eq!(g.query_root(4), 1);
        g.set_root(4);
        assert_eq!(g.query_root(1), 4);
        assert_eq!(g.query_root(2), 4);
        assert_eq!(g.query_root(3), 4);
        assert_eq!(g.query_root(4), 4);
    }

    #[test]
    fn test_add_two_parents() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        g.add_edge(3, 1);
        assert_eq!(g.query_root(3), 1);
        // Note that adding a second edge changes the root!
        g.add_edge(3, 2);
        assert_eq!(g.query_root(3), 2);
        assert_eq!(g.query_root(1), 2);
    }

    #[test]
    fn test_remove_edge() {
        let mut g = TopTrees::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        g.add_vertex(4);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 4);
        assert_eq!(g.query_root(1), 4);
        // Note that removing an edge changes the root!
        g.remove_edge(3, 2);
        assert_eq!(g.query_root(1), 2);
        assert_eq!(g.query_root(2), 2);
        assert_eq!(g.query_root(3), 3);
        assert_eq!(g.query_root(4), 3);
    }
}
