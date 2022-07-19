/*
    Wrapper trait for HashMap-like collections.

    Used to swap out different backend implementations easily.
    Primarily because repeated HashMap calls get extremely expensive for
    data structures based on a bunch of nodes pointing to other nodes,
    so it's nice to have a plain Vec representation.

    **Warning:** This does not make the same guarantees as HashMap as
    new Default values may be added to the map at will for efficiency.
    If tracking Some vs None is desired, use Option<V> for the value type!**

    Specifically:
    - valid_key(k) must return true if the key is safe to access
      (but may return true even for values not explicitly added previously)
    - ensure(k) ensures that k is a valid key and inserts the default value
    - index(k) gets the value at the index; may panic if not a valid key
      or may return the default value
    - index_mut(k) gets the value at the index mutably, may panic if not
      a valid key or may return the default value
    - iter() iterates over key-value pairs for debugging purposes

    Generally the assumption is index() and index_mut() are only called
    on ensured keys, and valid_key() and iter() are only used for debugging
    purposes. See avl_forest.rs for an example intended use.
*/

use std::collections::HashMap;
use std::hash::Hash;

pub trait Hashy<K, V>: Default {
    fn valid_key(&self, k: &K) -> bool;
    fn index(&self, k: &K) -> &V;
    fn index_mut(&mut self, k: &K) -> &mut V;
    fn ensure(&mut self, k: K)
    where
        V: Default;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (K, &'a V)> + 'a>;
}

/*
    Standard hashmap

    Performance:
    This implementation has a high constant overhead, but is
    space-efficient and generally reliable, serves as a good baseline.
*/
impl<K: Clone + Hash + Eq, V> Hashy<K, V> for HashMap<K, V> {
    fn valid_key(&self, k: &K) -> bool {
        Self::contains_key(self, k)
    }
    fn index(&self, k: &K) -> &V {
        self.get(k).unwrap()
    }
    fn index_mut(&mut self, k: &K) -> &mut V {
        self.get_mut(k).unwrap()
    }
    fn ensure(&mut self, k: K)
    where
        V: Default,
    {
        self.entry(k).or_insert_with(Default::default);
    }
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (K, &'a V)> + 'a> {
        Box::new(self.iter().map(|(k, v)| (k.clone(), v)))
    }
}

/*
    1D Vector-based hashmap

    Performance:
    This implementation is not heavily tested as it can't be used
    for avl_forest.rs.
*/
#[derive(Debug)]
pub struct VecMap1D<V>(Vec<Option<V>>);
impl<V> Default for VecMap1D<V> {
    fn default() -> Self {
        Self(Vec::new())
    }
}
impl<V: Clone> Hashy<usize, V> for VecMap1D<V> {
    fn valid_key(&self, &k: &usize) -> bool {
        k < self.0.len() && self.0[k].is_some()
    }
    fn index(&self, &k: &usize) -> &V {
        self.0[k].as_ref().unwrap()
    }
    fn index_mut(&mut self, &k: &usize) -> &mut V {
        self.0[k].as_mut().unwrap()
    }
    fn ensure(&mut self, k: usize)
    where
        V: Default,
    {
        if k >= self.0.len() {
            self.0.resize(k + 1, None);
        }
        if self.0[k].is_none() {
            self.0[k] = Some(Default::default());
        }
    }
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (usize, &'a V)> + 'a> {
        let result = self
            .0
            .as_slice()
            .iter()
            .enumerate()
            .filter_map(|(i, v)| Some(i).zip(v.as_ref()));
        Box::new(result)
    }
}

/*
    2D Vector-based hashmap

    Performance:
    This implementation is extremely space-hungry (O(n^2) in largest index,
    not necessarily contiguous memory regions)
    and crashes when indices get too large.
*/
#[derive(Debug)]
pub struct VecMap2D<V>(Vec<Vec<V>>);
const VECMAP_INIT_LEN: usize = 1000;
impl<V: Clone + Default> Default for VecMap2D<V> {
    fn default() -> Self {
        Self(vec![Default::default(); VECMAP_INIT_LEN])
    }
}
impl<V: Clone + Default> Hashy<(usize, usize), V> for VecMap2D<V> {
    fn valid_key(&self, &(i, j): &(usize, usize)) -> bool {
        i < self.0.len() && j < self.0[i].len()
    }
    fn index(&self, &(i, j): &(usize, usize)) -> &V {
        &self.0[i][j]
    }
    fn index_mut(&mut self, &(i, j): &(usize, usize)) -> &mut V {
        &mut self.0[i][j]
    }
    fn ensure(&mut self, (i, j): (usize, usize)) {
        if i >= self.0.len() {
            self.0.resize(i + 1, vec![Default::default(); self.0.len()]);
        }
        if j >= self.0[i].len() {
            self.0[i].resize_with(j + 1, Default::default);
        }
    }
    fn iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = ((usize, usize), &'a V)> + 'a> {
        Box::new(self.0.as_slice().iter().enumerate().flat_map(|(i, xs)| {
            xs.as_slice().iter().enumerate().map(move |(j, x)| ((i, j), x))
        }))
    }
}

/*
    1D Vector-based hashmap with 2D indices -- using a pairing function

    Performance:
    This implementation tries to fix the problems with VecMap2D by relying
    on a single giant vector, but it doesn't seem to work, it is still
    extremely space-hungry and slow when indices get too large.
*/
fn cantor_pair(i: usize, j: usize) -> usize {
    (i + j + 1) * (i + j) / 2 + i
}
fn undo_pair(k: usize) -> (usize, usize) {
    let w = ((((8 * k + 1) as f64).sqrt() - 1.0) / 2.0) as usize;
    let t = (w + 1) * w / 2;
    (k - t, w + t - k)
}

#[derive(Debug)]
pub struct VecMapP<V>(Vec<V>);
impl<V: Default> Default for VecMapP<V> {
    fn default() -> Self {
        Self(vec![Default::default()])
    }
}
impl<V: Clone + Default> Hashy<(usize, usize), V> for VecMapP<V> {
    fn valid_key(&self, &(i, j): &(usize, usize)) -> bool {
        // print!("v");
        cantor_pair(i, j) < self.0.len()
    }
    fn index(&self, &(i, j): &(usize, usize)) -> &V {
        // print!("g");
        let k = cantor_pair(i, j);
        debug_assert!(k < self.0.len());
        &self.0[k]
    }
    fn index_mut(&mut self, &(i, j): &(usize, usize)) -> &mut V {
        // print!("m");
        let k = cantor_pair(i, j);
        debug_assert!(k < self.0.len());
        &mut self.0[k]
    }
    fn ensure(&mut self, (i, j): (usize, usize)) {
        // print!("\nE ");
        let k = cantor_pair(i, j);
        debug_assert!(!self.0.is_empty());
        while k >= self.0.len() {
            // double size
            // println!("doubling...");
            self.0.resize_with(2 * self.0.len(), Default::default);
        }
    }
    fn iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = ((usize, usize), &'a V)> + 'a> {
        // print!("\nI ");
        Box::new(
            self.0
                .as_slice()
                .iter()
                .enumerate()
                .map(|(k, v)| (undo_pair(k), v)),
        )
    }
}

/*
    Vector-of-HashMaps "Hybrid" hashmap

    This is a more conventionally reasonable representation for
    sparse graph, much less space-hungry.

    Performance:
    This is the only implementation so far that manages to compete with just
    plain HashMap, and might be better. So let's use it for now.
*/
#[derive(Debug)]
pub struct VecMapHy<V>(Vec<HashMap<usize, V>>);
impl<V: Clone + Default> Default for VecMapHy<V> {
    fn default() -> Self {
        Self(vec![Default::default()])
    }
}
impl<V: Clone + Default> Hashy<(usize, usize), V> for VecMapHy<V> {
    fn valid_key(&self, &(i, j): &(usize, usize)) -> bool {
        i < self.0.len() && self.0[i].contains_key(&j)
    }
    fn index(&self, &(i, j): &(usize, usize)) -> &V {
        self.0[i].get(&j).unwrap()
    }
    fn index_mut(&mut self, &(i, j): &(usize, usize)) -> &mut V {
        self.0[i].get_mut(&j).unwrap()
    }
    fn ensure(&mut self, (i, j): (usize, usize)) {
        debug_assert!(!self.0.is_empty());
        while i >= self.0.len() {
            // double size
            self.0.resize_with(2 * self.0.len(), HashMap::new);
        }
        self.0[i].entry(j).or_insert_with(Default::default);
    }
    fn iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = ((usize, usize), &'a V)> + 'a> {
        Box::new(
            self.0
                .as_slice()
                .iter()
                .enumerate()
                .flat_map(|(i, xs)| xs.iter().map(move |(&j, x)| ((i, j), x))),
        )
    }
}

/*
    Unit tests
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantor_pairing() {
        assert_eq!(cantor_pair(0, 0), 0);
        assert_eq!(cantor_pair(0, 1), 1);
        assert_eq!(cantor_pair(1, 0), 2);
        assert_eq!(cantor_pair(0, 2), 3);
        assert_eq!(cantor_pair(1, 1), 4);
        assert_eq!(cantor_pair(2, 0), 5);
        assert_eq!(cantor_pair(0, 3), 6);
        assert_eq!(cantor_pair(1, 2), 7);
        assert_eq!(cantor_pair(2, 1), 8);
        assert_eq!(cantor_pair(3, 0), 9);
        assert_eq!(cantor_pair(0, 4), 10);
    }

    #[test]
    fn test_undo_pairing() {
        assert_eq!(undo_pair(0), (0, 0));
        assert_eq!(undo_pair(1), (0, 1));
        assert_eq!(undo_pair(2), (1, 0));
        assert_eq!(undo_pair(3), (0, 2));
        assert_eq!(undo_pair(4), (1, 1));
        assert_eq!(undo_pair(5), (2, 0));
        assert_eq!(undo_pair(6), (0, 3));
        assert_eq!(undo_pair(7), (1, 2));
        assert_eq!(undo_pair(8), (2, 1));
        assert_eq!(undo_pair(9), (3, 0));
        assert_eq!(undo_pair(10), (0, 4));
    }
}
