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

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::hash::Hash;

enum BinTree<V> {
    Leaf(V),
    Node(BTreeSet<BinTree<V>>),
}

pub struct TopTrees<V: Copy + Eq + Hash + Ord> {
    trees: BTreeMap<V, BinTree<V>>,
    nodes: HashSet<V>,
}
impl<V: Copy + Eq + Hash + Ord> TopTrees<V> {
    fn add_tree(&mut self, v: V) {
        debug_assert!(!self.nodes.contains(&v));
        debug_assert!(!self.trees.contains_key(&v));
        self.trees.insert(v, BinTree::Leaf(v));
        self.nodes.insert(v);
    }
    fn query_root(&self, v: V) -> V {
        unimplemented!()
    }
    fn same_root(&self, v1: V, v2: V) -> bool {
        self.query_root(v1) == self.query_root(v2)
    }
    fn set_root(&mut self, v: V) {
        unimplemented!()
    }
    fn join_trees(&mut self, v1: V, v2: V) {
        unimplemented!()
    }
    fn split_trees(&mut self, v1: V, v2: V) {
        unimplemented!()
    }
}
