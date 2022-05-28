/*
    Wrapper trait for hashmap-like collections.

    Used to swap out different backend implementations easily.
    Primarily because repeated HashMap calls get extremely expensive for
    data structures based on a bunch of nodes pointing to other nodes,
    so it's nice to have a plain Vec representation.

    This API is a work in progress and probably will change and be
    cleaned up later:
    - .get() functionality is not actually currently needed, only
      .get_unwrapped() and .get_mut_unwrapped().
      (See the only use of the file in avl_forest.rs)
    - The VecMap2D below makes different assumptions than the
      other implementations. It may insert default values into
      the map arbitrarily, whereas the other implementations
      use Option to track which values are seen / not seen.
      But this is again not needed by the current clients
      except for debugging purposes.
    - The trait is more or less an extension of Index and IndexMut
      and could just be named accordingly.
*/

use std::collections::HashMap;
use std::hash::Hash;

pub trait Hashy<K, V>: Default {
    fn contains_key(&self, k: &K) -> bool;
    // fn get(&self, k: &K) -> Option<&V>;
    // fn get_mut(&mut self, k: &K) -> Option<&mut V>;
    fn get_unwrapped(&self, k: &K) -> &V;
    fn get_mut_unwrapped(&mut self, k: &K) -> &mut V;
    // fn insert(&mut self, k: K, v: V);
    fn ensure(&mut self, k: K)
    where
        V: Default;
    // Iterator -- currently used for debugging purposes only,
    // so avoid having to do too much lifetime hacking we use
    // a Box dyn trait object.
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (K, &'a V)> + 'a>;
}

/*
    Standard hashmap
*/
impl<K: Clone + Hash + Eq, V> Hashy<K, V> for HashMap<K, V> {
    fn contains_key(&self, k: &K) -> bool {
        Self::contains_key(self, k)
    }
    // fn get(&self, k: &K) -> Option<&V> {
    //     HashMap::get(self, k)
    // }
    // fn get_mut(&mut self, k: &K) -> Option<&mut V> {
    //     HashMap::get_mut(self, k)
    // }
    fn get_unwrapped(&self, k: &K) -> &V {
        self.get(k).unwrap()
    }
    fn get_mut_unwrapped(&mut self, k: &K) -> &mut V {
        self.get_mut(k).unwrap()
    }
    // fn insert(&mut self, k: K, v: V) {
    //     HashMap::insert(self, k, v);
    // }
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
*/
#[derive(Debug)]
pub struct VecMap1D<V>(Vec<Option<V>>);
impl<V> Default for VecMap1D<V> {
    fn default() -> Self {
        Self(Vec::new())
    }
}
impl<V: Clone> Hashy<usize, V> for VecMap1D<V> {
    fn contains_key(&self, &k: &usize) -> bool {
        k < self.0.len() && self.0[k].is_some()
    }
    // fn get(&self, &k: &usize) -> Option<&V> {
    //     if k < self.0.len() {
    //         self.0[k].as_ref()
    //     } else {
    //         None
    //     }
    // }
    // fn get_mut(&mut self, &k: &usize) -> Option<&mut V> {
    //     if k < self.0.len() {
    //         self.0[k].as_mut()
    //     } else {
    //         None
    //     }
    // }
    fn get_unwrapped(&self, &k: &usize) -> &V {
        self.0[k].as_ref().unwrap()
    }
    fn get_mut_unwrapped(&mut self, &k: &usize) -> &mut V {
        self.0[k].as_mut().unwrap()
    }
    // fn insert(&mut self, k: usize, v: V) {
    //     if k >= self.0.len() {
    //         self.0.resize(k + 1, None);
    //     }
    //     self.0[k] = Some(v);
    // }
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
            .map(|(i, v)| Some(i).zip(v.as_ref()))
            .flatten();
        Box::new(result)
    }
}

/*
    2D Vector-based hashmap

    Warning: for efficiency reasons, this map has a slightly less strict API --
    it stores Default::<V>::default() items instead of None.
    So when using this, remember that contains_key is an overapproximation
    and get() may return a default element instead of None.
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
    fn contains_key(&self, &(i, j): &(usize, usize)) -> bool {
        // print!("c");
        i < self.0.len() && j < self.0[i].len()
    }
    // fn get(&self, &(i, j): &(usize, usize)) -> Option<&V> {
    //     // print!("g");
    //     if i < self.0.len() {
    //         if j < self.0.len() {
    //             Some(&self.0[i][j])
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }
    // fn get_mut(&mut self, &(i, j): &(usize, usize)) -> Option<&mut V> {
    //     // print!("m");
    //     if i < self.0.len() {
    //         if j < self.0.len() {
    //             Some(&mut self.0[i][j])
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }
    fn get_unwrapped(&self, &(i, j): &(usize, usize)) -> &V {
        // print!("G");
        &self.0[i][j]
    }
    fn get_mut_unwrapped(&mut self, &(i, j): &(usize, usize)) -> &mut V {
        // print!("M");
        &mut self.0[i][j]
    }
    // fn insert(&mut self, (i, j): (usize, usize), v: V) {
    //     // println!();
    //     // print!("i ");
    //     if i >= self.0.len() {
    //         self.0.resize(i + 1, vec![Default::default(); self.0.len()]);
    //     }
    //     if j >= self.0[i].len() {
    //         self.0[i].resize_with(j + 1, Default::default);
    //     }
    //     self.0[i][j] = v;
    // }
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
        // print!("t");
        Box::new(self.0.as_slice().iter().enumerate().flat_map(|(i, xs)| {
            xs.as_slice().iter().enumerate().map(move |(j, x)| ((i, j), x))
        }))
    }
}

/*
    1D Vector-based hashmap with 2D indices -- using a pairing function

    Makes the same assumptions as VecMap2D -- may insert additional
    Default values into the map.
*/
fn cantor_pair(i: usize, j: usize) -> usize {
    (i + j + 1) * (i + j) / 2 + i
}
fn undo_pair(k: usize) -> (usize, usize) {
    let w = ((((8 * k + 1) as f64).sqrt() - 1.0) / 2.0) as usize;
    let t = (w + 1) * w / 2;
    println!("{k}, {w}, {t}");
    (k - t, w + t - k)
}

#[derive(Debug)]
pub struct VecMapP<V>(Vec<V>);
impl<V> Default for VecMapP<V> {
    fn default() -> Self {
        Self(Vec::new())
    }
}
impl<V: Clone + Default> Hashy<(usize, usize), V> for VecMapP<V> {
    fn contains_key(&self, &(i, j): &(usize, usize)) -> bool {
        cantor_pair(i, j) < self.0.len()
    }
    fn get_unwrapped(&self, &(i, j): &(usize, usize)) -> &V {
        &self.0[cantor_pair(i, j)]
    }
    fn get_mut_unwrapped(&mut self, &(i, j): &(usize, usize)) -> &mut V {
        &mut self.0[cantor_pair(i, j)]
    }
    fn ensure(&mut self, (i, j): (usize, usize))
    where
        V: Default,
    {
        let k = cantor_pair(i, j);
        while k >= self.0.len() {
            // double size
            self.0.resize_with(self.0.len() * 2, Default::default);
        }
    }
    fn iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = ((usize, usize), &'a V)> + 'a> {
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
