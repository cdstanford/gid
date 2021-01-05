/*
    A simple directed graph implementation that is used by the different
    implementations of the state graph interface.

    Supports:
    - adding vertices with names of type V, labeled by type T
    - merging vertices in O(1) time (the two vertex names are now aliases)
      (requires a merge function T x T -> T)
    - iterating through the edges at a vertex (O(1) per edge)
*/

use super::debug_counter::DebugCounter;
use disjoint_sets::UnionFind;
use std::collections::{HashMap, LinkedList};
use std::hash::Hash;

// Newtypes to keep different types of ID straight
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct UniqueID(usize);
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct CanonicalID(usize);

#[derive(Debug)]
pub struct DiGraph<V, T> {
    vertex_ids: HashMap<V, UniqueID>,
    id_vertices: HashMap<UniqueID, V>,
    id_find: UnionFind<usize>,
    labels: HashMap<CanonicalID, T>,
    fwd_edges: HashMap<CanonicalID, LinkedList<UniqueID>>,
    bck_edges: HashMap<CanonicalID, LinkedList<UniqueID>>,
    // Debug mode statistics
    space: DebugCounter,
    time: DebugCounter,
}
impl<V, T> Default for DiGraph<V, T> {
    // Can't derive automatically because don't want to assume T: Default
    fn default() -> Self {
        Self {
            vertex_ids: Default::default(),
            id_vertices: Default::default(),
            id_find: Default::default(),
            labels: Default::default(),
            fwd_edges: Default::default(),
            bck_edges: Default::default(),
            space: Default::default(),
            time: Default::default(),
        }
    }
}
impl<V, T> DiGraph<V, T>
where
    V: Copy + Clone + Eq + Hash + PartialEq,
{
    /*
        Exposed API
    */
    pub fn new() -> Self {
        Default::default()
    }
    pub fn is_seen(&self, v: V) -> bool {
        self.time.inc();
        self.vertex_ids.contains_key(&v)
    }
    pub fn get_label(&self, v: V) -> Option<&T> {
        self.time.inc();
        self.get_canon_id(v).and_then(|id| self.labels.get(&id))
    }
    pub fn add_vertex(&mut self, v: V, label: T) {
        // overwrites if already seen
        if self.is_seen(v) {
            let canon_id = self.get_canon_id(v).unwrap();
            self.labels.insert(canon_id, label);
            self.time.inc();
        } else {
            let new_id = self.id_find.alloc();
            let unique_id = UniqueID(new_id);
            let canon_id = CanonicalID(new_id);
            debug_assert_eq!(self.id_find.find(new_id), new_id);
            debug_assert!(!self.vertex_ids.contains_key(&v));
            debug_assert!(!self.id_vertices.contains_key(&unique_id));
            debug_assert!(!self.labels.contains_key(&canon_id));
            debug_assert!(!self.fwd_edges.contains_key(&canon_id));
            debug_assert!(!self.bck_edges.contains_key(&canon_id));
            self.vertex_ids.insert(v, unique_id);
            self.id_vertices.insert(unique_id, v);
            self.labels.insert(canon_id, label);
            self.fwd_edges.insert(canon_id, LinkedList::new());
            self.bck_edges.insert(canon_id, LinkedList::new());
            self.time.inc();
            self.space.inc();
        }
    }
    pub fn add_edge(&mut self, v1: V, v2: V) {
        // Panics if v1 and v2 are not seen
        assert!(self.is_seen(v1));
        assert!(self.is_seen(v2));
        let canon1 = self.get_canon_id(v1).unwrap();
        let canon2 = self.get_canon_id(v2).unwrap();
        if canon1 != canon2 {
            self.fwd_edges
                .get_mut(&canon1)
                .unwrap()
                .push_back(UniqueID(canon2.0));
            self.bck_edges
                .get_mut(&canon2)
                .unwrap()
                .push_back(UniqueID(canon1.0));
            self.space.inc();
        }
        self.time.inc();
    }
    pub fn iter_vertices<'a>(&'a self) -> impl Iterator<Item = V> + 'a {
        // TODO: iterate through each canonical ID only once instead
        self.vertex_ids.keys().copied().inspect(move |_| self.time.inc())
    }
    pub fn iter_bck_edges<'a>(&'a self, v: V) -> impl Iterator<Item = V> + 'a {
        assert!(self.is_seen(v));
        let canon = self.get_canon_id(v).unwrap();
        self.bck_edges[&canon]
            .iter()
            .map(move |id| self.id_find.find(id.0))
            .map(move |id| self.id_vertices.get(&UniqueID(id)).unwrap())
            .copied()
            .inspect(move |_| self.time.inc())
    }
    // TODO: Merge vertices
    // pub fn merge(&mut self, v1: V, v2: V) {
    // }

    /*
        Debug mode statistics
        These panic if not in debug mode.
    */
    // These panic if not in debug mode.
    pub fn get_space(&self) -> usize {
        self.space.get()
    }
    pub fn get_time(&self) -> usize {
        self.time.get()
    }

    /*
        Internal
    */
    fn get_canon_id(&self, v: V) -> Option<CanonicalID> {
        self.vertex_ids
            .get(&v)
            .map(|id| self.id_find.find(id.0))
            .map(CanonicalID)
    }
}
