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

// TODO: remove after implementing basic version
#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct TopTrees<V: Copy + Debug + Eq + Hash + Ord> {
    parents: HashMap<V, Option<V>>,
}
impl<V: Copy + Debug + Eq + Hash + Ord> TopTrees<V> {
    pub fn add_vertex(&mut self, v: V) {
        assert!(!self.is_seen(v));
        self.parents.insert(v, None);
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
            debug_assert!(self.get_parent(next).is_none());
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
        debug_assert!(self.get_parent(v1).is_none());
        self.set_parent(v1, v2);
    }
    pub fn remove_edge(&mut self, v1: V, v2: V) {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        assert_eq!(self.get_parent(v1), Some(v2));
        debug_assert!(self.same_root(v1, v2));
        self.erase_parent(v1);
    }

    /*
        Internal
    */
    fn is_seen(&self, v: V) -> bool {
        self.parents.contains_key(&v)
    }
    fn get_parent(&self, v: V) -> Option<V> {
        assert!(self.is_seen(v));
        *self.parents.get(&v).unwrap()
    }
    fn set_parent(&mut self, v1: V, v2: V) {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        *self.parents.get_mut(&v1).unwrap() = Some(v2);
    }
    fn erase_parent(&mut self, v1: V) {
        assert!(self.is_seen(v1));
        *self.parents.get_mut(&v1).unwrap() = None;
    }
    fn is_root(&self, v: V) -> bool {
        assert!(self.is_seen(v));
        self.query_root(v) == v
    }
    fn same_root(&self, v1: V, v2: V) -> bool {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        self.query_root(v1) == self.query_root(v2)
    }
}
