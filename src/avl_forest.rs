/*
    AVL Forest

    Data structure mainaining a collection of balanced AVL trees;
    unlike the usual AVL trees, nodes are not ordered by key
    and we maintain many trees (a forest) at once,
    but the basic algorithms and rebalancing operations are the same.

    Semantically, each AVL tree in the collection is an ordered list.
    The data structure supports the following in O(log n):
    - ensure(x): add a new tree with a single node x, if x doesn't yet exist
    - get_root(x): get the root node for the AVL tree containing x
    - concat(x, y): concatenate AVL trees containing x and y
    - split(x): Split the AVL tree containing x, removing x from its tree.

    Example:
        ensure(1), ensure(3), ensure(2)
            [1] [3] [2]
        concat(1, 3)
            [1, 3] [2]
        concat(1, 2) (or concat(3, 2))
            [1, 3, 2]
        split(1)
            [1], [3, 2]
        split(3)
            [1], [3], [2]

    Also, split(3) from [1, 3, 2] yields [1], [3], [2] directly.

    Also exposes public APIs for:
    - is_seen(x): true if ensure(x) has been called previously
    - next(x) and prev(x): next and previous vertices from x
      e.g. in [1, 3, 2], next(3) = 2, prev(3) = 1, next(2) = None
    - an iterator repeatedly calling next(x)

    This data structure is used for connectivity in undirected forests,
    a la Henzinger-King (Euler tour trees). Introduced in:
        Randomized Fully Dynamic Graph Algorithms with Polylogarithmic Time
        per Operation. Monika R. Henzinger and Valerie King, JACM 1999.

    Helpful notes on this:
        http://courses.csail.mit.edu/6.851/spring07/scribe/lec05.pdf
*/

use super::debug_counter::DebugCounter;
use super::hashy::{Hashy, VecMap1D, VecMap2D, VecMapP};
use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Node<V> {
    height: usize,
    parent: Option<V>,
    lchild: Option<V>,
    rchild: Option<V>,
}
impl<V> Default for Node<V> {
    fn default() -> Self {
        Self { height: 1, parent: None, lchild: None, rchild: None }
    }
}

/*
    Generic implementation for any "hashy" data structure H --
    this allows different backends other than just HashMap
*/
#[derive(Debug)]
pub struct AvlForest<V, H>
where
    V: Copy + Debug + Eq,
    H: Hashy<V, Node<V>>,
{
    nodes: H,
    _phantom_v: PhantomData<V>,
    time: DebugCounter,
    space: DebugCounter,
}
impl<V, H> Default for AvlForest<V, H>
where
    V: Copy + Debug + Eq,
    H: Hashy<V, Node<V>>,
{
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            _phantom_v: Default::default(),
            time: Default::default(),
            space: Default::default(),
        }
    }
}
impl<V, H> AvlForest<V, H>
where
    V: Copy + Debug + Eq,
    H: Hashy<V, Node<V>>,
{
    /*
        Primary public API
    */
    pub fn new() -> Self {
        // println!("new()");
        let result: Self = Default::default();
        result.assert_invariant();
        result
    }
    pub fn ensure(&mut self, v: V) {
        // println!("ensure({v:?})");
        self.time.inc();
        self.space.inc();
        self.nodes.ensure(v);
        self.assert_invariant();
    }
    pub fn get_root(&self, mut v: V) -> V {
        // println!("get_root({v:?})");
        // Running time O(h) in the height of the tree h
        debug_assert!(self.is_seen(v));
        self.time.inc();
        while let Some(parent) = self.node_parent(v) {
            self.time.inc();
            v = parent
        }
        v
    }
    pub fn concat(&mut self, v1: V, v2: V) -> bool {
        // println!("concat({v1:?}, {v2:?})");
        // Return true if successful
        debug_assert!(self.is_seen(v1));
        debug_assert!(self.is_seen(v2));
        self.time.inc();

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
    pub fn split(&mut self, v: V) {
        // println!("split({:?}", v);
        debug_assert!(self.is_seen(v));
        // println!("Splitting on: {:?}", v);
        self.time.inc();

        let mut lsplit: Option<V> = self.detach_lchild(v);
        let mut rsplit: Option<V> = self.detach_rchild(v);
        self.set_height(v);
        debug_assert_eq!(self.height(v), 1);

        // Travel upward from v, on each upwards-left move add to lsplit,
        // on each upwards-right move add to rsplit.
        let mut pivot = v;
        let mut next_parent = self.node_parent(v);
        self.node_mut(v).parent = None;
        while let Some(p) = next_parent {
            self.time.inc();
            next_parent = self.node_parent(p);
            self.node_mut(p).parent = None;

            if self.node(p).rchild == Some(pivot) {
                self.set_rchild(p, lsplit);
                lsplit = Some(self.rebalance_full(p));
            } else {
                debug_assert_eq!(self.node(p).lchild, Some(pivot));
                self.set_lchild(p, rsplit);
                rsplit = Some(self.rebalance_full(p));
            }

            pivot = p;
        }

        self.assert_invariant();
    }

    /*
        Additional publicly exposed functions
    */
    pub fn same_root(&self, v1: V, v2: V) -> bool {
        // println!("same_root({:?}, {:?})", v1, v2);
        debug_assert!(self.is_seen(v1));
        debug_assert!(self.is_seen(v2));
        self.time.inc();

        self.get_root(v1) == self.get_root(v2)
    }
    pub fn is_seen(&self, v: V) -> bool {
        // println!("is_seen({:?})", v);
        self.nodes.valid_key(&v)
    }
    pub fn next(&self, mut v: V) -> Option<V> {
        // println!("next({v:?})");
        self.time.inc();
        if let Some(mut c) = self.node(v).rchild {
            while let Some(cnew) = self.node(c).lchild {
                self.time.inc();
                c = cnew;
            }
            return Some(c);
        }
        while let Some(par) = self.node(v).parent {
            self.time.inc();
            if self.node(par).lchild == Some(v) {
                return Some(par);
            }
            v = par;
        }
        None
    }
    pub fn prev(&self, mut v: V) -> Option<V> {
        // println!("prev({v:?})");
        self.time.inc();
        if let Some(mut c) = self.node(v).lchild {
            while let Some(cnew) = self.node(c).rchild {
                self.time.inc();
                c = cnew;
            }
            return Some(c);
        }
        while let Some(par) = self.node(v).parent {
            self.time.inc();
            if self.node(par).rchild == Some(v) {
                return Some(par);
            }
            v = par;
        }
        None
    }

    /*
        Public getters for debugging only
    */
    pub fn iter_fwd_from(&self, v: V) -> impl Iterator<Item = V> + '_ {
        // println!("iter_fwd_from({v:?})");
        iter::successors(Some(v), move |&v| self.next(v))
    }
    pub fn get_time(&self) -> usize {
        // println!("get_time()");
        self.time.get()
    }
    pub fn get_space(&self) -> usize {
        // println!("get_space()");
        self.space.get()
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
        self.time.inc();

        let n1 = self.node(r1);
        let n2 = self.node(r2);
        match n1.height.cmp(&n2.height) {
            Ordering::Greater => {
                if let Some(c1) = n1.rchild {
                    self.node_mut(c1).parent = None;
                    r2 = self.concat_roots(c1, r2);
                }
                self.set_rchild(r1, Some(r2));
                self.rebalance_rheavy(r1)
            }
            Ordering::Less => {
                if let Some(c2) = n2.lchild {
                    self.node_mut(c2).parent = None;
                    r1 = self.concat_roots(r1, c2);
                }
                self.set_lchild(r2, Some(r1));
                self.rebalance_lheavy(r2)
            }
            Ordering::Equal => {
                let (head, tail) = self.pop_front(r2);
                self.set_lchild(head, Some(r1));
                self.set_rchild(head, tail);
                // Should not need rebalancing
                self.set_height(head);
                debug_assert!(self.is_balanced(head));
                head
            }
        }
    }

    /*
        Internal accessors
    */
    fn node(&self, v: V) -> &Node<V> {
        self.nodes.index(&v)
    }
    fn node_parent(&self, v: V) -> Option<V> {
        self.node(v).parent
    }

    /*
        Internal modifiers
        (not necessarily preserving data structure invariants)
    */
    fn node_mut(&mut self, v: V) -> &mut Node<V> {
        self.nodes.index_mut(&v)
    }
    fn set_rchild(&mut self, p: V, c: Option<V>) {
        self.time.inc();
        self.node_mut(p).rchild = c;
        if let Some(c0) = c {
            self.node_mut(c0).parent = Some(p);
        }
    }
    fn set_lchild(&mut self, p: V, c: Option<V>) {
        self.time.inc();
        self.node_mut(p).lchild = c;
        if let Some(c0) = c {
            self.node_mut(c0).parent = Some(p);
        }
    }
    fn detach_lchild(&mut self, p: V) -> Option<V> {
        self.time.inc();
        let c = self.node(p).lchild;
        if let Some(c0) = c {
            self.node_mut(p).lchild = None;
            self.node_mut(c0).parent = None;
        }
        c
    }
    fn detach_rchild(&mut self, p: V) -> Option<V> {
        self.time.inc();
        let c = self.node(p).rchild;
        if let Some(c0) = c {
            self.node_mut(p).rchild = None;
            self.node_mut(c0).parent = None;
        }
        c
    }
    fn pop_front(&mut self, rt: V) -> (V, Option<V>) {
        // Pop the leftmost element of the tree containing v, returning it
        // (the head) and the remaining tree (tail)
        // Precondition: rt is a root
        debug_assert_eq!(self.node_parent(rt), None);
        self.time.inc();

        if let Some(c) = self.detach_lchild(rt) {
            let (head, tail) = self.pop_front(c);
            self.set_lchild(rt, tail);
            let tail = self.rebalance_rheavy(rt);
            (head, Some(tail))
        } else {
            let c = self.detach_rchild(rt);
            self.set_height(rt);
            (rt, c)
        }
    }

    /*
        Height computations

        None is defined to be height 0, a node with no children
        is height 1, and so on.
    */
    fn height(&self, v: V) -> usize {
        self.node(v).height
    }
    fn height_opt(&self, child: Option<V>) -> usize {
        child.map_or(0, |v| self.node(v).height)
    }
    fn child_heights(&self, v: V) -> (usize, usize) {
        let n = self.node(v);
        let h1 = self.height_opt(n.lchild);
        let h2 = self.height_opt(n.rchild);
        (h1, h2)
    }
    fn compute_height(&self, v: V) -> usize {
        let (h1, h2) = self.child_heights(v);
        1 + cmp::max(h1, h2)
    }
    fn set_height(&mut self, v: V) {
        self.node_mut(v).height = self.compute_height(v);
    }

    /*
        AVL balancing operations
    */
    fn is_balanced(&self, v: V) -> bool {
        // Mostly for debugging purposes.
        let n = self.node(v);
        let h = n.height;
        let h1 = self.height_opt(n.lchild);
        let h2 = self.height_opt(n.rchild);
        (h == 1 + cmp::max(h1, h2)) && (h1 <= h2 + 1) && (h2 <= h1 + 1)
    }
    fn rebalance_lheavy(&mut self, mut v: V) -> V {
        // O(1) rebalance at v
        // Preconditions:
        // - v is a root, but height may not be set correctly
        // - right <= left + 1, left <= right + 2
        debug_assert_eq!(self.node_parent(v), None);
        let (h1, h2) = self.child_heights(v);
        debug_assert!(h2 <= h1 + 1);
        debug_assert!(h1 <= h2 + 2);
        self.set_height(v);
        self.time.inc();

        if h1 == h2 + 2 {
            let c1 = self.node(v).lchild.unwrap();
            let (h11, h12) = self.child_heights(c1);
            if h11 < h12 {
                debug_assert_eq!(h11 + 1, h12);
                debug_assert_eq!(h11, h2);
                let c1 = self.rotate_left(c1);
                self.set_lchild(v, Some(c1));
                v = self.rotate_right(v);
                debug_assert!(self.is_balanced(self.node(v).lchild.unwrap()));
                debug_assert!(self.is_balanced(self.node(v).rchild.unwrap()));
            } else {
                v = self.rotate_right(v);
            }
        }
        debug_assert!(self.is_balanced(v));
        v
    }
    fn rebalance_rheavy(&mut self, mut v: V) -> V {
        // O(1) rebalance at v
        // Preconditions:
        // - v is a root, but height may not be set correctly
        // - left <= right + 1, right <= left + 2
        debug_assert_eq!(self.node_parent(v), None);
        let (h1, h2) = self.child_heights(v);
        debug_assert!(h1 <= h2 + 1);
        debug_assert!(h2 <= h1 + 2);
        self.set_height(v);
        self.time.inc();

        if h2 == h1 + 2 {
            let c2 = self.node(v).rchild.unwrap();
            let (h21, h22) = self.child_heights(c2);
            if h21 > h22 {
                debug_assert_eq!(h22 + 1, h21);
                debug_assert_eq!(h22, h1);
                let c2 = self.rotate_right(c2);
                self.set_rchild(v, Some(c2));
                v = self.rotate_left(v);
                debug_assert!(self.is_balanced(self.node(v).lchild.unwrap()));
                debug_assert!(self.is_balanced(self.node(v).rchild.unwrap()));
            } else {
                v = self.rotate_left(v);
            }
        }
        debug_assert!(self.is_balanced(v));
        v
    }
    fn rebalance_full(&mut self, mut v: V) -> V {
        // println!("rebalance full: {:?} ({:?})", v, self.node(v));
        // O(log n) rebalance at v, assuming nothing about the two
        // children except that they are balanced subtrees.
        // For now, I'm just implementing this by calling concat.
        // There might be a slightly better way.
        self.time.inc();
        let c1 = self.detach_lchild(v);
        let c2 = self.detach_rchild(v);
        self.set_height(v);
        if let Some(c) = c1 {
            v = self.concat_roots(c, v);
        }
        if let Some(c) = c2 {
            v = self.concat_roots(v, c);
        }
        debug_assert!(self.is_balanced(v));
        v
    }
    fn rotate_right(&mut self, v: V) -> V {
        self.time.inc();
        let left = self.detach_lchild(v).unwrap();
        let mid = self.detach_rchild(left);
        self.set_lchild(v, mid);
        self.set_rchild(left, Some(v));
        self.set_height(v);
        self.set_height(left);
        left
    }
    fn rotate_left(&mut self, v: V) -> V {
        self.time.inc();
        let right = self.detach_rchild(v).unwrap();
        let mid = self.detach_lchild(right);
        self.set_rchild(v, mid);
        self.set_lchild(right, Some(v));
        self.set_height(v);
        self.set_height(right);
        right
    }

    /*
        Data structure invariant
    */
    #[cfg(debug_assertions)]
    fn assert_invariant(&self) {
        for (v, node) in self.nodes.iter() {
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
            // Height is correct and balanced
            // println!("{:?}", self);
            assert!(self.is_balanced(v));
        }
    }
    #[cfg(not(debug_assertions))]
    fn assert_invariant(&self) {}
}

/*
    Specializations with particular HashMap implementation backings
*/
pub type AvlForestHM<V> = AvlForest<V, HashMap<V, Node<V>>>;
pub type AvlForest1DVec = AvlForest<usize, VecMap1D<Node<usize>>>;
pub type AvlForest2DVec =
    AvlForest<(usize, usize), VecMap2D<Node<(usize, usize)>>>;
pub type AvlForestPVec =
    AvlForest<(usize, usize), VecMapP<Node<(usize, usize)>>>;

/*
    Unit tests
*/
#[cfg(test)]
mod tests {
    use super::*;

    // Most examples use the AvlForestHM backend.
    // A few examples at the end use AvlForest1DVec.
    // Would be good to have some AvlForest2DVec examples.

    impl<V, H> AvlForest<V, H>
    where
        V: Copy + Debug + Eq,
        H: Hashy<V, Node<V>>,
    {
        fn collect_succs(&mut self, v: V) -> Vec<V> {
            self.iter_fwd_from(v).collect()
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
        let mut forest = AvlForestHM::new();
        forest.ensure(2);
        forest.ensure(2);
        forest.ensure(3);
        forest.ensure(5);
        assert_eq!(forest.get_root(2), 2);
        assert_eq!(forest.get_root(3), 3);
        assert_eq!(forest.get_root(5), 5);
    }

    #[test]
    #[should_panic]
    fn test_get_root_nonexistent() {
        let mut forest = AvlForestHM::new();
        forest.ensure(2);
        forest.ensure(2);
        forest.get_root(1);
    }

    #[test]
    fn test_concat_simple() {
        let mut forest = AvlForestHM::new();
        forest.ensure('a');
        forest.ensure('b');
        assert!(forest.concat('a', 'b'));
    }

    #[test]
    fn test_concat() {
        let mut forest = AvlForestHM::new();
        forest.ensure(2);
        forest.ensure(4);
        forest.ensure(6);

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
        let mut forest = AvlForestHM::new();
        forest.ensure(0);
        assert_eq!(forest.collect_succs(0), vec![0]);
        for i in 1..=10 {
            forest.ensure(i);
            assert!(forest.concat(0, i));
            assert_eq!(forest.collect_succs(0), range_vec(0, i));
            assert_eq!(forest.collect_succs(i), vec![i]);
        }
    }

    #[test]
    fn test_concat_repeat_prepend() {
        let mut forest = AvlForestHM::new();
        forest.ensure(0);
        assert_eq!(forest.collect_succs(0), vec![0]);
        for i in 1..=10 {
            forest.ensure(i);
            assert!(forest.concat(i, 0));
            assert_eq!(forest.collect_succs(i), range_rev_vec(0, i));
            assert_eq!(forest.collect_succs(0), vec![0]);
        }
    }

    #[test]
    fn test_concat_doubling() {
        let mut forest = AvlForestHM::new();
        for i in 0..=7 {
            forest.ensure(i);
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
        let mut forest = AvlForestHM::new();
        forest.ensure(3);
        forest.ensure(2);
        forest.ensure(1);
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
    fn test_split_at_simple_1() {
        let mut forest = AvlForestHM::new();
        forest.ensure('a');
        forest.ensure('b');

        println!("=== concat(a, b) ===");
        assert!(forest.concat('a', 'b'));
        assert_eq!(forest.collect_succs('a'), vec!['a', 'b']);

        println!("=== split(a) ===");
        forest.split('a');
        assert_eq!(forest.collect_succs('a'), vec!['a']);
        assert_eq!(forest.get_root('a'), 'a');
        assert_eq!(forest.get_root('b'), 'b');

        println!("=== concat(b, a) ===");
        assert!(forest.concat('b', 'a'));
        assert_eq!(forest.collect_succs('b'), vec!['b', 'a']);

        println!("=== split(a) ===");
        forest.split('a');
        assert_eq!(forest.collect_succs('b'), vec!['b']);
    }

    #[test]
    fn test_split_at_simple_2() {
        let mut forest = AvlForestHM::new();
        forest.ensure('a');
        forest.ensure('b');
        forest.ensure('c');

        assert!(forest.concat('a', 'b'));
        assert!(forest.concat('a', 'c'));
        assert_eq!(forest.collect_succs('a'), vec!['a', 'b', 'c']);

        forest.split('a');
        assert_eq!(forest.collect_succs('a'), vec!['a']);
        assert_eq!(forest.collect_succs('b'), vec!['b', 'c']);

        forest.split('b');
        assert_eq!(forest.collect_succs('a'), vec!['a']);
        assert_eq!(forest.collect_succs('b'), vec!['b']);
        assert_eq!(forest.collect_succs('c'), vec!['c']);
    }

    #[test]
    fn test_split_at_simple_3() {
        let mut forest = AvlForestHM::new();
        forest.ensure('a');
        forest.ensure('b');
        forest.ensure('c');
        assert!(forest.concat('a', 'b'));
        assert!(forest.concat('a', 'c'));

        forest.split('b');
        assert_eq!(forest.collect_succs('a'), vec!['a']);
        assert_eq!(forest.collect_succs('b'), vec!['b']);
        assert_eq!(forest.collect_succs('c'), vec!['c']);
    }

    fn n_chain(n: usize) -> AvlForest1DVec {
        let mut forest = AvlForest1DVec::new();
        forest.ensure(1);
        for i in 2..=n {
            forest.ensure(i);
            forest.concat(i - 1, i);
        }
        forest
    }

    #[test]
    fn test_split_bigchain() {
        const BIG: usize = 10;
        println!("===== Big chain {BIG} =====");
        for i in 1..=BIG {
            println!("=== splitting at {i}");
            let mut forest = n_chain(BIG);
            forest.split(i);
            if i > 1 {
                assert_eq!(forest.collect_succs(1), range_vec(1, i - 1));
                assert_eq!(forest.collect_succs(i - 1), vec![i - 1]);
            }
            assert_eq!(forest.collect_succs(i), vec![i]);
            if i < BIG {
                assert_eq!(forest.collect_succs(i + 1), range_vec(i + 1, BIG));
                assert_eq!(forest.collect_succs(BIG), vec![BIG]);
            }
        }
    }

    #[test]
    fn test_next_prev() {
        let forest = n_chain(10);
        for i in 1..=9 {
            assert_eq!(forest.next(i), Some(i + 1));
            assert_eq!(forest.prev(i + 1), Some(i));
        }
        assert_eq!(forest.prev(1), None);
        assert_eq!(forest.next(10), None);
    }
}
