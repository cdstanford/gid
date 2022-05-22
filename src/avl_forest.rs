/*
    AVL Forest

    Data structure mainaining a collection of balanced AVL trees;
    unlike the usual AVL trees, nodes are not ordered by key
    and we maintain many trees (a forest) at once,
    but the basic algorithms and rebalancing operations are the same.

    Semantically, each AVL tree in the collection is an ordered list.
    The data structure supports the following in O(log n):
    - add(x): add a new tree with a single node x
    - root(x): get the root node for the AVL tree containing x
    - concat(x, y): concatenate AVL trees containing x and y
    - split(x): Split the AVL tree containing x after x.

    Example:
        add(1), add(3), add(2)
            [1] [3] [2]
        concat(1, 3)
            [1, 3] [2]
        concat(1, 2) (or concat(3, 2))
            [1, 3, 2]
        split(1)
            [1], [3, 2]
        split(3)
            [1], [3], [2]

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
    height: usize,
    parent: Option<V>,
    lchild: Option<V>,
    rchild: Option<V>,
}
impl<V: IdType> Default for Node<V> {
    fn default() -> Self {
        Self { height: 0, parent: None, lchild: None, rchild: None }
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
        Public API
    */
    pub fn new() -> Self {
        let result: Self = Default::default();
        result.assert_invariant();
        result
    }
    pub fn ensure_vertex(&mut self, v: V) {
        if !self.is_seen(v) {
            self.nodes.insert(v, Default::default());
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
    pub fn split_after(&mut self, mut v: V) {
        debug_assert!(self.is_seen(v));
        // println!("{:?}, splitting after {:?}", self, v);

        let mut lsplit: Option<V> = Some(v);
        let mut rsplit: Option<V> = self.detach_rchild(v);
        self.set_height(v);

        // Travel upward from v, on each upwards-left move add to lsplit,
        // on each upwards-right move add to rsplit.
        while let Some(p) = self.node_parent(v) {
            self.node_mut(v).parent = None;

            if self.node(p).rchild == Some(v) {
                self.set_rchild(p, lsplit);
                lsplit = Some(p);
            } else {
                debug_assert_eq!(self.node(p).lchild, Some(v));
                self.set_lchild(p, rsplit);
                rsplit = Some(p);
            }

            self.set_height(p);
            v = p;
        }

        self.assert_invariant();
    }
    pub fn iter_succs(&self, v: V) -> impl Iterator<Item = V> + '_ {
        iter::successors(Some(v), move |&v| self.succ(v))
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
            self.set_rchild(r1, Some(r2));
            self.set_height(r1);
            r1
        } else {
            if let Some(c2) = n2.lchild {
                self.node_mut(c2).parent = None;
                r1 = self.concat_roots(r1, c2);
            }
            self.set_lchild(r2, Some(r1));
            self.set_height(r2);
            r2
        }
    }

    /*
        Basic accessors
    */
    fn is_seen(&self, v: V) -> bool {
        self.nodes.contains_key(&v)
    }
    fn node(&self, v: V) -> &Node<V> {
        self.nodes.get(&v).unwrap()
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
        Internal setters
    */
    fn node_mut(&mut self, v: V) -> &mut Node<V> {
        self.nodes.get_mut(&v).unwrap()
    }
    fn set_rchild(&mut self, p: V, c: Option<V>) {
        self.node_mut(p).rchild = c;
        if let Some(c0) = c {
            self.node_mut(c0).parent = Some(p);
        }
    }
    fn set_lchild(&mut self, p: V, c: Option<V>) {
        self.node_mut(p).lchild = c;
        if let Some(c0) = c {
            self.node_mut(c0).parent = Some(p);
        }
    }
    fn detach_rchild(&mut self, p: V) -> Option<V> {
        let c = self.node(p).rchild;
        if let Some(c0) = c {
            self.node_mut(p).rchild = None;
            self.node_mut(c0).parent = None;
        }
        c
    }

    /*
        Internal AVL balancing operations
    */
    fn height_above(&self, child: Option<V>) -> usize {
        child.map_or(0, |v| self.node(v).height + 1)
    }
    // TODO uncomment
    // fn is_balanced(&self, n: &Node<V>) -> bool {
    //     let h1 = self.height_above(n.lchild);
    //     let h2 = self.height_above(n.rchild);
    //     (h1 <= h2 + 1) && (h2 <= h1 + 1)
    // }
    fn compute_height(&self, n: &Node<V>) -> usize {
        let h1 = self.height_above(n.lchild);
        let h2 = self.height_above(n.rchild);
        // Balance check
        // TODO uncomment once implemented
        // debug_assert!(self.is_balanced(n));
        h1.max(h2)
    }
    fn set_height(&mut self, v: V) {
        self.node_mut(v).height = self.compute_height(self.node(v));
    }
    // TODO

    /*
        Data structure invariant
    */
    #[cfg(debug_assertions)]
    fn assert_invariant(&self) {
        for (&v, node) in self.nodes.iter() {
            // Parent points to children
            if let Some(p) = node.parent {
                assert!(self.is_seen(p));
                let n = self.node(p);
                assert!(n.lchild == Some(v) || n.rchild == Some(v));
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
            // Balanced
            // TODO uncomment once implemented
            // assert!(self.is_balanced(node));
        }
    }
    #[cfg(not(debug_assertions))]
    fn assert_invariant(&self) {}
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

    fn range_vec(i: usize, j: usize) -> Vec<usize> {
        (i..=j).collect()
    }
    fn range_rev_vec(i: usize, j: usize) -> Vec<usize> {
        (i..=j).rev().collect()
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

        // forest: [4], [2], [6]
        assert!(!forest.same_root(2, 4));
        assert!(!forest.same_root(2, 6));
        assert_eq!(forest.collect_succs(6), vec![6]);
        assert_eq!(forest.collect_succs(2), vec![2]);
        assert_eq!(forest.collect_succs(4), vec![4]);

        assert!(forest.concat(4, 2));
        // forest: [4, 2], [6]
        assert!(forest.same_root(2, 4));
        assert!(!forest.same_root(2, 6));
        assert_eq!(forest.collect_succs(6), vec![6]);
        assert_eq!(forest.collect_succs(2), vec![2]);
        assert_eq!(forest.collect_succs(4), vec![4, 2]);

        assert!(forest.concat(4, 6));
        // forest:
        assert!(forest.same_root(2, 4));
        assert!(forest.same_root(2, 6));
        assert_eq!(forest.collect_succs(6), vec![6]);
        assert_eq!(forest.collect_succs(2), vec![2, 6]);
        assert_eq!(forest.collect_succs(4), vec![4, 2, 6]);
    }

    #[test]
    fn test_concat_repeat_append() {
        let mut forest = AvlForest::new();
        forest.ensure_vertex(0);
        assert_eq!(forest.collect_succs(0), vec![0]);
        for i in 1..=10 {
            forest.ensure_vertex(i);
            assert!(forest.concat(0, i));
            assert_eq!(forest.collect_succs(0), range_vec(0, i));
            assert_eq!(forest.collect_succs(i), vec![i]);
        }
    }

    #[test]
    fn test_concat_repeat_prepend() {
        let mut forest = AvlForest::new();
        forest.ensure_vertex(0);
        assert_eq!(forest.collect_succs(0), vec![0]);
        for i in 1..=10 {
            forest.ensure_vertex(i);
            assert!(forest.concat(i, 0));
            assert_eq!(forest.collect_succs(i), range_rev_vec(0, i));
            assert_eq!(forest.collect_succs(0), vec![0]);
        }
    }

    #[test]
    fn test_concat_doubling() {
        let mut forest = AvlForest::new();
        for i in 0..=7 {
            forest.ensure_vertex(i);
        }
        assert!(forest.concat(0, 1));
        assert!(forest.concat(2, 3));
        assert!(forest.concat(4, 5));
        assert!(forest.concat(6, 7));
        assert_eq!(forest.collect_succs(0), vec![0, 1]);
        assert_eq!(forest.collect_succs(2), vec![2, 3]);
        assert_eq!(forest.collect_succs(4), vec![4, 5]);
        assert_eq!(forest.collect_succs(6), vec![6, 7]);
        assert!(forest.concat(1, 5));
        assert!(forest.concat(2, 7));
        assert_eq!(forest.collect_succs(0), vec![0, 1, 4, 5]);
        assert_eq!(forest.collect_succs(2), vec![2, 3, 6, 7]);
        assert!(forest.concat(3, 4));
        assert_eq!(forest.collect_succs(2), vec![2, 3, 6, 7, 0, 1, 4, 5]);
    }

    #[test]
    fn test_concat_unsuccessful() {
        let mut forest = AvlForest::new();
        forest.ensure_vertex(3);
        forest.ensure_vertex(2);
        forest.ensure_vertex(1);
        assert!(!forest.concat(2, 2));

        assert!(forest.concat(1, 2));
        assert!(!forest.concat(2, 1));
        assert!(!forest.concat(1, 2));
        assert!(!forest.concat(3, 3));

        assert!(forest.concat(2, 3));
        assert!(!forest.concat(3, 2));
        assert!(!forest.concat(1, 3));
        assert!(!forest.concat(3, 1));
    }

    #[test]
    fn test_split_after_simple_1() {
        let mut forest = AvlForest::new();
        forest.ensure_vertex('a');
        forest.ensure_vertex('b');

        assert!(forest.concat('a', 'b'));
        assert_eq!(forest.collect_succs('a'), vec!['a', 'b']);
        forest.split_after('b');
        assert_eq!(forest.collect_succs('a'), vec!['a', 'b']);
        assert_eq!(forest.get_root('a'), forest.get_root('b'));
        forest.split_after('a');
        assert_eq!(forest.collect_succs('a'), vec!['a']);
        assert_eq!(forest.get_root('a'), 'a');
        assert_eq!(forest.get_root('b'), 'b');

        assert!(forest.concat('b', 'a'));
        assert_eq!(forest.collect_succs('b'), vec!['b', 'a']);
        forest.split_after('b');
        assert_eq!(forest.collect_succs('b'), vec!['b']);
    }

    #[test]
    fn test_split_after_simple_2() {
        let mut forest = AvlForest::new();
        forest.ensure_vertex('a');
        forest.ensure_vertex('b');
        forest.ensure_vertex('c');

        assert!(forest.concat('a', 'b'));
        assert!(forest.concat('a', 'c'));
        assert_eq!(forest.collect_succs('a'), vec!['a', 'b', 'c']);

        forest.split_after('a');
        assert_eq!(forest.collect_succs('a'), vec!['a']);
        assert_eq!(forest.collect_succs('b'), vec!['b', 'c']);

        forest.split_after('b');
        assert_eq!(forest.collect_succs('a'), vec!['a']);
        assert_eq!(forest.collect_succs('b'), vec!['b']);
        assert_eq!(forest.collect_succs('c'), vec!['c']);
    }

    #[test]
    fn test_split_after_simple_3() {
        let mut forest = AvlForest::new();
        forest.ensure_vertex('a');
        forest.ensure_vertex('b');
        forest.ensure_vertex('c');
        assert!(forest.concat('a', 'b'));
        assert!(forest.concat('a', 'c'));

        forest.split_after('b');
        assert_eq!(forest.collect_succs('a'), vec!['a', 'b']);
        assert_eq!(forest.collect_succs('c'), vec!['c']);

        forest.split_after('a');
        assert_eq!(forest.collect_succs('a'), vec!['a']);
        assert_eq!(forest.collect_succs('b'), vec!['b']);
        assert_eq!(forest.collect_succs('c'), vec!['c']);
    }

    fn n_chain(n: usize) -> AvlForest<usize> {
        let mut forest = AvlForest::new();
        forest.ensure_vertex(1);
        for i in 2..=n {
            forest.ensure_vertex(i);
            forest.concat(i - 1, i);
        }
        forest
    }

    #[test]
    fn test_split_bigchain() {
        const BIG: usize = 10;
        for i in 1..=BIG {
            let mut forest = n_chain(BIG);
            forest.split_after(i);
            assert_eq!(forest.collect_succs(1), range_vec(1, i));
            if i < BIG {
                assert_eq!(forest.collect_succs(i + 1), range_vec(i + 1, BIG));
            }
            assert_eq!(forest.collect_succs(i), vec![i]);
            assert_eq!(forest.collect_succs(BIG), vec![BIG]);
        }
    }
}
