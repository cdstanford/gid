/*
    Implementation of Frederickson's Topology Trees
    for dynamic undirected graph connectivity problems.

    The data structure maintains a forest graph, and supports
    the following in O(log n) per operation:
    - Adding a new 1-vertex tree
    - Checking whether two vertices are in the same tree
    - Joining two trees into one by adding an edge
    - Splitting a tree into two by removing an edge

    This data structure can be used efficiently for undirected
    connectivity in forests. It doesn't solve the problem of
    undirected connectivity in *general* graphs, but forests
    will be enough for our use case.

    TODO List:
    1. implement naive version that works but isn't efficient
        ==> Done
    2. write unit tests
        ==> Done
    3. implement the efficient version

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

/*
    Internal types used by TopTrees
*/
// Trait bound abbreviation
pub trait IdType: Copy + Debug + Eq + Hash + Ord {}
impl<I: Copy + Debug + Eq + Hash + Ord> IdType for I {}

// Type used to identify nodes -- can be replaced with anything that
// identifies an edge or vertex uniquely.
#[derive(Copy, Debug, Eq, Hash, Ord, Clone, PartialEq, PartialOrd)]
enum NodeId<V: IdType> {
    Edge(V, V),
    Vert(V),
}

// Represents a node in a balanced hierarchical decomposition of a forest
// where each vertex has degree at most 3. TopTrees will do the work of
// converting to degree 3.
// N will be instantiated with NodeId for now, but can be any unique
// identifier for each node.
#[derive(Debug, Clone)]
enum NodeCase<V: IdType, N: IdType> {
    SplitOnEdge(N, N),
    SingleVertex(V),
}
#[derive(Debug, Clone)]
struct Node<V: IdType, N: IdType> {
    id: N,
    parent: Option<N>,
    kind: NodeCase<V, N>,
}

/*
    The publicly exposed data structure
*/
#[derive(Debug, Clone)]
pub struct TopTrees<V: IdType> {
    nodes: HashMap<NodeId<V>, Node<V, NodeId<V>>>,

    parents: HashMap<V, Option<V>>,
}
impl<V: IdType> Default for TopTrees<V> {
    fn default() -> Self {
        let nodes = Default::default();
        let parents = Default::default();
        Self { nodes, parents }
    }
}
impl<V: IdType> TopTrees<V> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn ensure_vertex(&mut self, v: V) {
        if !self.is_seen(v) {
            self.parents.insert(v, None);
            let id = NodeId::Vert(v);
            let parent = None;
            let kind = NodeCase::SingleVertex(v);
            let node = Node { id, parent, kind };
            self.nodes.insert(id, node);
        }
    }
    pub fn add_edge(&mut self, v1: V, v2: V) {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        assert!(!self.same_root(v1, v2));

        let n1 = self.node_root(NodeId::Vert(v1));
        let n2 = self.node_root(NodeId::Vert(v2));

        // New SplitOnEdge node to join two trees
        let id = NodeId::Edge(v1, v2);
        let parent = None;
        let kind = NodeCase::<V, NodeId<V>>::SplitOnEdge(n1, n2);
        let node = Node { id, parent, kind };
        self.nodes.insert(id, node);

        // Set parents
        debug_assert!(self.node_mut(n1).parent.is_none());
        debug_assert!(self.node_mut(n2).parent.is_none());
        self.node_mut(n1).parent = Some(id);
        self.node_mut(n2).parent = Some(id);

        // Old code, to remove when the new impl works
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
    pub fn same_root(&self, v1: V, v2: V) -> bool {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        let result = self.same_root_1(v1, v2);
        debug_assert_eq!(result, self.same_root_2(v1, v2));
        result
    }
    fn same_root_1(&self, v1: V, v2: V) -> bool {
        self.query_root(v1) == self.query_root(v2)
    }
    fn same_root_2(&self, v1: V, v2: V) -> bool {
        let n1 = self.node_root(NodeId::Vert(v1));
        let n2 = self.node_root(NodeId::Vert(v2));
        n1 == n2
    }

    /*
        Internal
    */
    // Basic primitives
    fn is_seen(&self, v: V) -> bool {
        self.nodes.contains_key(&NodeId::Vert(v))
    }
    fn get_parent(&self, v: V) -> Option<V> {
        assert!(self.is_seen(v));
        *self.parents.get(&v).unwrap()
    }
    fn is_root(&self, v: V) -> bool {
        assert!(self.is_seen(v));
        self.get_parent(v).is_none()
    }
    // Node moidifiers
    fn node(&self, n: NodeId<V>) -> &Node<V, NodeId<V>> {
        self.nodes.get(&n).unwrap()
    }
    fn node_mut(&mut self, n: NodeId<V>) -> &mut Node<V, NodeId<V>> {
        self.nodes.get_mut(&n).unwrap()
    }
    fn node_parent(&self, n: NodeId<V>) -> Option<NodeId<V>> {
        self.node(n).parent
    }
    fn node_root(&self, mut n: NodeId<V>) -> NodeId<V> {
        while let Some(parent) = self.node_parent(n) {
            n = parent
        }
        n
    }
    fn node_left(&self, n: NodeId<V>) -> Option<NodeId<V>> {
        match self.node(n).kind {
            NodeCase::SplitOnEdge(n1, _) => Some(n1),
            NodeCase::SingleVertex(_) => None,
        }
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
    // Root querying / modification
    fn query_root(&self, v: V) -> V {
        let result = self.query_root_1(v);
        // debug_assert_eq(result, self.query_root_2(v));
        result
    }
    fn query_root_1(&self, mut v: V) -> V {
        assert!(self.is_seen(v));
        while !self.is_root(v) {
            v = self.get_parent(v).unwrap();
        }
        v
    }
    fn query_root_2(&self, v: V) -> V {
        assert!(self.is_seen(v));
        let mut n = NodeId::Vert(v);
        while let Some(n1) = self.node_parent(n) {
            n = n1;
        }
        while let Some(n1) = self.node_left(n) {
            n = n1;
        }
        match self.node(n).kind {
            NodeCase::SplitOnEdge(_, _) => unreachable!(),
            NodeCase::SingleVertex(v) => {
                debug_assert_eq!(n, NodeId::Vert(v));
                v
            }
        }
    }
    fn set_root(&mut self, v: V) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vertex() {
        let mut g = TopTrees::new();
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
        let g = TopTrees::new();
        g.is_root(1);
    }

    #[test]
    #[should_panic]
    fn test_query_nonexistent_2() {
        let g = TopTrees::new();
        g.same_root(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_edge_nonexistent_1() {
        let mut g = TopTrees::new();
        g.ensure_vertex(1);
        g.add_edge(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_edge_nonexistent_2() {
        let mut g = TopTrees::new();
        g.ensure_vertex(2);
        g.add_edge(1, 2);
    }

    #[test]
    fn test_two_vertices() {
        let mut g = TopTrees::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.add_edge(1, 2);
        assert!(g.same_root(1, 2));
        assert!(g.same_root(2, 1));
    }

    #[test]
    #[should_panic]
    fn test_add_edge_twice() {
        let mut g = TopTrees::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.add_edge(1, 2);
        g.add_edge(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_add_self_edge() {
        let mut g = TopTrees::new();
        g.ensure_vertex(1);
        g.add_edge(1, 1);
    }

    #[test]
    fn test_add_edges() {
        let mut g = TopTrees::new();
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
        let mut g = TopTrees::new();
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

        // assert_eq!(g.query_root(0), 8);
        // assert_eq!(g.query_root(1), 8);
        // assert_eq!(g.query_root(2), 8);
        // assert_eq!(g.query_root(3), 8);
        // assert_eq!(g.query_root(4), 7);
        // assert_eq!(g.query_root(5), 7);
        // assert_eq!(g.query_root(6), 7);
        // assert_eq!(g.query_root(7), 7);
        // assert_eq!(g.query_root(8), 8);
        // assert_eq!(g.query_root(9), 8);
    }

    #[test]
    #[should_panic]
    fn test_add_cycle_2() {
        let mut g = TopTrees::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.add_edge(1, 2);
        g.add_edge(2, 1);
    }

    #[test]
    #[should_panic]
    fn test_add_cycle_3() {
        let mut g = TopTrees::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.ensure_vertex(3);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 1);
    }

    #[test]
    #[should_panic]
    fn test_add_cycle_4() {
        let mut g = TopTrees::new();
        g.ensure_vertex(1);
        g.ensure_vertex(2);
        g.ensure_vertex(3);
        g.ensure_vertex(4);
        g.add_edge(1, 2);
        g.add_edge(3, 4);
        g.add_edge(2, 3);
        g.add_edge(4, 1);
    }

    // #[test]
    // fn test_set_root_1() {
    //     let mut g = TopTrees::new();
    //     g.ensure_vertex(1);
    //     g.ensure_vertex(2);
    //     g.add_edge(1, 2);
    //     g.set_root(1);
    //     assert_eq!(g.query_root(1), 1);
    //     assert_eq!(g.query_root(2), 1);
    // }

    // #[test]
    // fn test_set_root_2() {
    //     let mut g = TopTrees::new();
    //     g.ensure_vertex(1);
    //     g.ensure_vertex(2);
    //     g.ensure_vertex(3);
    //     g.ensure_vertex(4);
    //     g.add_edge(1, 2);
    //     g.add_edge(4, 3);
    //     g.add_edge(2, 3);
    //     assert_eq!(g.query_root(1), 3);
    //     assert_eq!(g.query_root(2), 3);
    //     assert_eq!(g.query_root(3), 3);
    //     assert_eq!(g.query_root(4), 3);
    //     g.set_root(1);
    //     assert_eq!(g.query_root(1), 1);
    //     assert_eq!(g.query_root(2), 1);
    //     assert_eq!(g.query_root(3), 1);
    //     assert_eq!(g.query_root(4), 1);
    //     g.set_root(4);
    //     assert_eq!(g.query_root(1), 4);
    //     assert_eq!(g.query_root(2), 4);
    //     assert_eq!(g.query_root(3), 4);
    //     assert_eq!(g.query_root(4), 4);
    // }

    #[test]
    fn test_add_two_parents() {
        let mut g = TopTrees::new();
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
    fn test_remove_edge() {
        let mut g = TopTrees::new();
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
        g.remove_edge(3, 2);
        assert!(g.same_root(1, 2));
        assert!(g.same_root(3, 4));
        assert!(!g.same_root(2, 3));
    }
}
