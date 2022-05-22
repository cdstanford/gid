/*
    AVL Forest

    Data structure mainaining a collection of balanced AVL trees;
    unlike the usual AVL trees, nodes are not ordered by key
    and we maintain many trees (a forest) at once,
    but the basic algorithms and rebalancing operations are the same.

    Semantically, each AVL tree in the collection is an ordered list.
    The data structure supports the following in O(log n):
    - root(x): get a canonical node for the AVL tree containing x
    - concat(x, y): concatenate AVL trees containing x and y
    - split(x): Split the AVL tree containing x after x.

    This data structure is used for connectivity in undirected forests,
    a la Henzinger-King (Euler tour trees). Introduced in:
        Randomized Fully Dynamic Graph Algorithms with Polylogarithmic Time
        per Operation. Monika R. Henzinger and Valerie King, JACM 1999.

    Helpful notes on this:
        http://courses.csail.mit.edu/6.851/spring07/scribe/lec05.pdf
*/

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;

// Trait bound abbreviation
pub trait IdType: Copy + Debug + Eq + Hash {}
impl<I: Copy + Debug + Eq + Hash> IdType for I {}

#[derive(Debug, Clone)]
struct Node<V: IdType> {
    label: V,
    height: usize,
    parent: Option<V>,
    lchild: Option<V>,
    rchild: Option<V>,
}
impl<V: IdType> Node<V> {
    fn new(v: V) -> Self {
        Self { label: v, height: 0, parent: None, lchild: None, rchild: None }
    }
}

#[derive(Debug)]
pub struct AvlForest<V: IdType> {
    nodes: HashMap<V, Node<V>>,
}
impl<V: IdType> Default for AvlForest<V> {
    fn default() -> Self {
        Self { nodes: Default::default() }
    }
}
impl<V: IdType> AvlForest<V> {
    /*
        Constructor and invariant
    */
    pub fn new() -> Self {
        let result: Self = Default::default();
        result.assert_invariant();
        result
    }
    #[cfg(debug_assertions)]
    fn assert_invariant(&self) {
        for (&v, node) in self.nodes.iter() {
            assert_eq!(v, node.label);
            // Parent exists
            if let Some(p) = node.parent {
                assert!(self.is_seen(p));
            }
            // Children point to parent
            if let Some(v1) = node.lchild {
                assert_eq!(self.node(v1).parent, Some(v));
            }
            if let Some(v2) = node.rchild {
                assert_eq!(self.node(v2).parent, Some(v));
            }
            // Height is correct
            assert_eq!(node.height, self.compute_height(node));
            // TODO: when balanced, also check heights are balanced
        }
    }
    #[cfg(not(debug_assertions))]
    fn assert_invariant(&self) {}

    /*
        Public API
    */
    pub fn ensure_vertex(&mut self, v: V) {
        if !self.is_seen(v) {
            self.nodes.insert(v, Node::new(v));
        }
        self.assert_invariant();
    }
    pub fn get_root(&self, mut v: V) -> V {
        // Running time O(h) in the height of the tree h
        debug_assert!(self.is_seen(v));
        while let Some(parent) = self.node_parent(v) {
            v = parent
        }
        v
    }
    pub fn same_root(&self, v1: V, v2: V) -> bool {
        debug_assert!(self.is_seen(v1));
        debug_assert!(self.is_seen(v2));
        self.get_root(v1) == self.get_root(v2)
    }
    pub fn concat(&mut self, v1: V, v2: V) -> bool {
        // Return true if successful
        debug_assert!(self.is_seen(v1));
        debug_assert!(self.is_seen(v2));
        let r1 = self.get_root(v1);
        let r2 = self.get_root(v2);
        if r1 == r2 {
            false
        } else {
            self.concat_roots(r1, r2);
            self.assert_invariant();
            true
        }
    }
    pub fn split_after(&mut self, v: V) {
        debug_assert!(self.is_seen(v));
        todo!()
        // self.assert_invariant();
    }

    /*
        Concatenate two trees at the roots, returning the new root.
        Recursive function called by concat
    */
    fn concat_roots(&mut self, mut r1: V, mut r2: V) -> V {
        // println!("Concat roots: {r1:?} {r2:?}");
        debug_assert!(r1 != r2);
        debug_assert_eq!(self.node_parent(r1), None);
        debug_assert_eq!(self.node_parent(r2), None);

        let n1 = self.node(r1);
        let n2 = self.node(r2);
        if n1.height >= n2.height {
            if let Some(c1) = n1.rchild {
                self.node_mut(c1).parent = None;
                r2 = self.concat_roots(c1, r2);
            }
            self.node_mut(r2).parent = Some(r1);
            self.node_mut(r1).rchild = Some(r2);
            self.set_height(r1);
            r1
        } else {
            if let Some(c2) = n2.lchild {
                self.node_mut(c2).parent = None;
                r1 = self.concat_roots(r1, c2);
            }
            self.node_mut(r1).parent = Some(r2);
            self.node_mut(r2).lchild = Some(r1);
            self.set_height(r2);
            r2
        }
    }

    /*
        Iterator
    */
    pub fn iter_succs(&self, v: V) -> impl Iterator<Item = V> + '_ {
        iter::successors(Some(v), move |&v| self.succ(v))
    }

    /*
        Basic accessors (internal)
    */
    fn is_seen(&self, v: V) -> bool {
        self.nodes.contains_key(&v)
    }
    fn node(&self, v: V) -> &Node<V> {
        self.nodes.get(&v).unwrap()
    }
    fn node_mut(&mut self, v: V) -> &mut Node<V> {
        self.nodes.get_mut(&v).unwrap()
    }
    fn node_parent(&self, v: V) -> Option<V> {
        self.node(v).parent
    }
    fn succ(&self, mut v: V) -> Option<V> {
        // println!("succ {v:?}");
        if let Some(mut c) = self.node(v).rchild {
            while let Some(cnew) = self.node(c).lchild {
                c = cnew;
            }
            return Some(c);
        }
        while let Some(par) = self.node(v).parent {
            if self.node(par).lchild == Some(v) {
                return Some(par);
            }
            v = par;
        }
        None
    }

    /*
        AVL balancing operations (internal)
    */
    fn compute_height(&self, n: &Node<V>) -> usize {
        let mut max_height: usize = 0;
        if let Some(v1) = n.lchild {
            max_height = max_height.max(self.node(v1).height + 1)
        }
        if let Some(v2) = n.rchild {
            max_height = max_height.max(self.node(v2).height + 1)
        }
        max_height
    }
    fn set_height(&mut self, v: V) {
        self.node_mut(v).height = self.compute_height(self.node(v));
    }
    // TODO
}

/*
    Unit tests
*/
#[cfg(test)]
mod tests {
    use super::*;

    impl<V: IdType> AvlForest<V> {
        fn collect_succs(&mut self, v: V) -> Vec<V> {
            self.iter_succs(v).collect()
        }
    }

    #[test]
    fn test_singletons() {
        let mut forest = AvlForest::new();
        forest.ensure_vertex(2);
        forest.ensure_vertex(2);
        forest.ensure_vertex(3);
        forest.ensure_vertex(5);
        assert_eq!(forest.get_root(2), 2);
        assert_eq!(forest.get_root(3), 3);
        assert_eq!(forest.get_root(5), 5);
    }

    #[test]
    #[should_panic]
    fn test_get_root_nonexistent() {
        let mut forest = AvlForest::new();
        forest.ensure_vertex(2);
        forest.ensure_vertex(2);
        forest.get_root(1);
    }

    #[test]
    fn test_concat() {
        let mut forest = AvlForest::new();
        forest.ensure_vertex(2);
        forest.ensure_vertex(4);
        forest.ensure_vertex(6);
        assert_eq!(forest.concat(2, 2), false);
        assert_eq!(forest.concat(4, 2), true);
        assert_eq!(forest.concat(2, 4), false);
        assert_eq!(forest.concat(4, 6), true);

        assert!(forest.same_root(2, 4));
        assert!(forest.same_root(2, 6));

        assert_eq!(forest.collect_succs(6), vec![6]);
        assert_eq!(forest.collect_succs(2), vec![2, 6]);
        assert_eq!(forest.collect_succs(4), vec![4, 2, 6]);
    }
}
