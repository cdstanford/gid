/*
    Generic search functions
*/

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Peekable;

/*
    Iterator for visiting items of type V in a DFS order.

    new() is given a set of source nodes 'start' and a 'next_nodes' function
    which is an abstract edge relation. The iterator iterates over all items of
    type V reachable using the 'next' function, not including those in 'start',
    and including each item only once.
*/
#[derive(Debug)]
pub struct DepthFirstSearch<V, I, F>
where
    V: Copy + Debug + Eq + Hash + PartialEq,
    I: Iterator<Item = V>,
    F: Fn(V) -> I,
{
    next_nodes: F,
    visited: HashSet<V>,
    frontier: Vec<I>,
}
impl<V, I, F> DepthFirstSearch<V, I, F>
where
    V: Copy + Debug + Eq + Hash + PartialEq,
    F: Fn(V) -> I,
    I: Iterator<Item = V>,
{
    pub fn new(start: impl Iterator<Item = V>, next_nodes: F) -> Self {
        let mut visited = HashSet::new();
        let mut frontier = Vec::new();
        for v in start {
            visited.insert(v);
            frontier.push(next_nodes(v));
        }
        Self { next_nodes, visited, frontier }
    }
}
impl<V, I, F> Iterator for DepthFirstSearch<V, I, F>
where
    V: Copy + Debug + Eq + Hash + PartialEq,
    F: Fn(V) -> I,
    I: Iterator<Item = V>,
{
    type Item = V;
    fn next(&mut self) -> Option<V> {
        while let Some(mut i) = self.frontier.pop() {
            while let Some(v) = i.next() {
                if !self.visited.contains(&v) {
                    self.frontier.push(i);
                    self.frontier.push((self.next_nodes)(v));
                    self.visited.insert(v);
                    return Some(v);
                }
            }
        }
        None
    }
}

/*
    Iterator for visiting items of type V in a topologically sorted order.

    This is a partial search: in the case of cycles, it will terminate after
    the largest initial segment of the graph that is acyclic, as there is no
    way to continue the topologically ordered search.

    Also, it is parameterized by a set of "candidate start" nodes, and will
    only consider the segment of the graph starting from those.

    new() is given:
    - the set of candidate start nodes 'start'
    - a 'next_nodes' function which is an abstract edge relation
    - a 'prev_nodes' function which is the reverse edge relation.

    Formally speaking, the iterator iterates over all nodes such that ALL
    maximal paths to that node are finite and start at a node in 'start'.
    Alternatively it can be thought of as a least-fixed-point computation to
    find a set of nodes closed under the prev_nodes relation, starting from
    one phantom node which points to all nodes in 'start'.
*/
#[derive(Debug)]
pub struct TopologicalSearch<V, I0, I1, I2, F1, F2>
where
    V: Copy + Eq + Hash + PartialEq,
    I0: Iterator<Item = V>,
    I1: Iterator<Item = V>,
    I2: Iterator<Item = V>,
    F1: Fn(V) -> I1,
    F2: Fn(V) -> I2,
{
    next_nodes: F1,
    prev_nodes: F2,
    visited: HashSet<V>,
    // The frontier is a bit more complex here.
    // start is intuitively thought of as the first element of frontier_fwd,
    // but kept separately in case I0 and I1 are different. An alternative
    // would be to make frontier_fwd of type Vec<Box<dyn Iterator<Item = V>>>,
    // but I prefer to avoid dyn trait objects.
    // start + frontier_fwd is a vector of the current DFS path.
    // frontier_bck checks for each possible next node whether all previous
    // nodes have been visited, and thus whether it is OK to visit next.
    start: I0,
    frontier_fwd: Vec<I1>,
    frontier_bck: HashMap<V, Peekable<I2>>,
}
impl<V, I0, I1, I2, F1, F2> TopologicalSearch<V, I0, I1, I2, F1, F2>
where
    V: Copy + Debug + Eq + Hash + PartialEq,
    I0: Iterator<Item = V>,
    I1: Iterator<Item = V>,
    I2: Iterator<Item = V>,
    F1: Fn(V) -> I1,
    F2: Fn(V) -> I2,
{
    pub fn new(start: I0, next_nodes: F1, prev_nodes: F2) -> Self {
        let visited = HashSet::new();
        let frontier_fwd = Vec::new();
        let frontier_bck = HashMap::new();
        Self {
            next_nodes,
            prev_nodes,
            visited,
            start,
            frontier_fwd,
            frontier_bck,
        }
    }
    fn can_visit(&mut self, v: V) -> bool {
        // Check if v is ready to be visited (i.e. all prev nodes visited)
        // (and update tracking of which prev node is waiting to be visited)
        // println!("[topsearch] can we visit {:?}?", v);
        if self.visited.contains(&v) {
            // println!("[topsearch] already visited {:?}", v);
            return false;
        }
        let iter_bck = {
            let temp1 = &mut self.frontier_bck;
            let temp2 = &self.prev_nodes;
            temp1.entry(v).or_insert_with(|| (temp2)(v).peekable())
        };
        while let Some(&u) = iter_bck.peek() {
            // println!("[topsearch] peeking at {:?}", u);
            if !self.visited.contains(&u) {
                // println!("[topsearch] not ready to visit {:?}", v);
                // println!("[topsearch] (must first visit {:?})", u);
                return false;
            }
            // println!("[topsearch] already visited {:?}", u);
            iter_bck.next();
        }
        // println!("[topsearch] ready to visit {:?}", v);
        true
    }
    fn visit(&mut self, v: V) -> Option<V> {
        // println!("[topsearch] visiting {:?}", v);
        self.visited.insert(v);
        self.frontier_fwd.push((self.next_nodes)(v));
        Some(v)
    }
}
impl<V, I0, I1, I2, F1, F2> Iterator
    for TopologicalSearch<V, I0, I1, I2, F1, F2>
where
    V: Copy + Debug + Eq + Hash + PartialEq,
    I0: Iterator<Item = V>,
    I1: Iterator<Item = V>,
    I2: Iterator<Item = V>,
    F1: Fn(V) -> I1,
    F2: Fn(V) -> I2,
{
    type Item = V;
    fn next(&mut self) -> Option<V> {
        // println!("[topsearch] NEXT");
        loop {
            while let Some(mut i) = self.frontier_fwd.pop() {
                // println!("[topsearch] trying frontier");
                while let Some(v) = i.next() {
                    if self.can_visit(v) {
                        self.frontier_fwd.push(i);
                        return self.visit(v);
                    }
                }
            }
            if let Some(v) = self.start.next() {
                // println!("[topsearch] trying start candidate: {:?}", v);
                if self.can_visit(v) {
                    return self.visit(v);
                }
            } else {
                // println!("[topsearch] nothing left, returning None");
                return None;
            }
        }
    }
}
