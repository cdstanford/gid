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
        ==> In Progress
        - Implemented first cut using unbalanced tree

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
    id: NodeId<V>,
    parent: Option<N>,
    kind: NodeCase<V, N>,
}
impl<V: IdType, N: IdType> Node<V, N> {
    #[cfg(debug_assertions)]
    fn assert_invariant(&self) {
        match self.kind {
            NodeCase::SplitOnEdge(_, _) => match self.id {
                NodeId::Edge(_, _) => (),
                NodeId::Vert(_) => panic!("invariant failed"),
            },
            NodeCase::SingleVertex(_) => match self.id {
                NodeId::Edge(_, _) => panic!("invariant failed"),
                NodeId::Vert(_) => (),
            },
        }
    }
    #[cfg(not(debug_assertions))]
    fn assert_invariant(&self) {}
    fn children(&self) -> Option<(N, N)> {
        self.assert_invariant();
        match self.kind {
            NodeCase::SplitOnEdge(n1, n2) => Some((n1, n2)),
            NodeCase::SingleVertex(_) => None,
        }
    }
    fn get_edge(&self) -> Option<(V, V)> {
        self.assert_invariant();
        match self.id {
            NodeId::Edge(v1, v2) => Some((v1, v2)),
            NodeId::Vert(_) => None,
        }
    }
}

/*
    The publicly exposed data structure
*/
#[derive(Debug, Clone)]
pub struct TopTrees<V: IdType> {
    nodes: HashMap<NodeId<V>, Node<V, NodeId<V>>>,
}
impl<V: IdType> Default for TopTrees<V> {
    fn default() -> Self {
        Self { nodes: Default::default() }
    }
}
impl<V: IdType> TopTrees<V> {
    pub fn new() -> Self {
        let result: Self = Default::default();
        result.assert_invariant();
        result
    }
    pub fn ensure_vertex(&mut self, v: V) {
        if !self.is_seen(v) {
            let id = NodeId::Vert(v);
            let parent = None;
            let kind = NodeCase::SingleVertex(v);
            let node = Node { id, parent, kind };
            self.nodes.insert(id, node);
        }

        self.assert_invariant();
    }
    pub fn add_edge(&mut self, v1: V, v2: V) {
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        assert!(!self.same_root(v1, v2));

        let n1 = self.get_root(v1);
        let n2 = self.get_root(v2);

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

        self.assert_invariant();
    }
    pub fn remove_edge(&mut self, v1: V, v2: V) {
        // TODO: this code is horrendous. Clean it up later if it works.

        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));

        let mut n = NodeId::Edge(v1, v2);
        debug_assert!(self.node_is_seen(n));
        let (mut n1, mut n2) = self.node(n).children().unwrap();

        self.node_mut(n1).parent = None;
        self.node_mut(n2).parent = None;

        let mut next_join = self.node_parent(n);
        self.nodes.remove(&n);

        while let Some(to_join) = next_join {
            // Invariant:
            // Two tree nodes, n1 and n2, need parents.
            // to_join is labeled with an edge and one of its children is n;
            // that child needs to be replaced with either n1 or n2.

            let (c1, c2) = self.node(to_join).children().unwrap();
            let (v1, v2) = self.node(to_join).get_edge().unwrap();

            if n == c1 {
                if self.get_root(v1) != n1 {
                    debug_assert_eq!(self.get_root(v1), n2);
                    std::mem::swap(&mut n1, &mut n2);
                }
                // replace c1 with n1
                self.node_mut(n1).parent = Some(to_join);
                self.node_mut(to_join).kind = NodeCase::SplitOnEdge(n1, c2);
                // Now par needs a parent
                n = to_join;
                n1 = to_join;
            } else {
                debug_assert_eq!(n, c2);
                // Same as previous case but for c2, v2 instead of c1, v1
                if self.get_root(v2) != n2 {
                    debug_assert_eq!(self.get_root(v2), n1);
                    std::mem::swap(&mut n1, &mut n2);
                }
                // replace c2 with n2
                self.node_mut(n2).parent = Some(to_join);
                self.node_mut(to_join).kind = NodeCase::SplitOnEdge(c1, n2);
                // Now par needs a parent
                n = to_join;
                n2 = to_join;
            }

            next_join = self.node_parent(n);
            self.node_mut(n).parent = None;
        }

        self.assert_invariant();
    }
    pub fn same_root(&self, v1: V, v2: V) -> bool {
        let n1 = self.get_root(v1);
        let n2 = self.get_root(v2);
        n1 == n2
    }

    /*
        Internal
    */
    // Invariant
    #[cfg(debug_assertions)]
    fn assert_invariant(&self) {
        for (&id, node) in self.nodes.iter() {
            node.assert_invariant();
            assert_eq!(id, node.id);
            if let Some(par) = node.parent {
                let (c1, c2) = self.node(par).children().unwrap();
                assert!(id == c1 || id == c2);
            }
            if let Some((n1, n2)) = node.children() {
                assert_eq!(id, self.node(n1).parent.unwrap());
                assert_eq!(id, self.node(n2).parent.unwrap());
            }
        }
    }
    #[cfg(not(debug_assertions))]
    fn assert_invariant(&self) {}
    // Basic primitives
    fn is_seen(&self, v: V) -> bool {
        self.node_is_seen(NodeId::Vert(v))
    }
    fn node_is_seen(&self, n: NodeId<V>) -> bool {
        self.nodes.contains_key(&n)
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
    fn get_root(&self, v: V) -> NodeId<V> {
        let mut n = NodeId::Vert(v);
        while let Some(parent) = self.node_parent(n) {
            n = parent
        }
        n
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
        g.same_root(1, 1);
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
    fn test_remove_edge_1() {
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
        g.remove_edge(2, 3);
        assert!(g.same_root(1, 2));
        assert!(g.same_root(3, 4));
        assert!(!g.same_root(2, 3));
    }

    #[test]
    fn test_remove_edge_2() {
        let mut g = TopTrees::new();
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

    // TODO: write some better test_remove_edge tests.
}
