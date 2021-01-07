/*
    Generic search functions
*/

use std::collections::HashSet;
use std::hash::Hash;

/*
    Iterator for visiting items of type V in a DFS order.

    new() is given a set of source vertices 'start' and a 'next_nodes' function
    which is an abstract edge relation. The iterator iterates over all items of
    type V reachable using the 'next' function, not including those in 'start',
    and including each item only once.
*/
pub struct DFS<V, F, I>
where
    V: Copy + Eq + Hash + PartialEq,
    F: Fn(V) -> I,
    I: Iterator<Item = V>,
{
    visited: HashSet<V>,
    next_nodes: F,
    frontier: Vec<I>,
}
impl<V, F, I> DFS<V, F, I>
where
    V: Copy + Eq + Hash + PartialEq,
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
        Self { visited, next_nodes, frontier }
    }
}
impl<V, F, I> Iterator for DFS<V, F, I>
where
    V: Copy + Eq + Hash + PartialEq,
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
