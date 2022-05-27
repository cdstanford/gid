/*
    Trait for hashmap-like collections.
*/

use std::collections::HashMap;
use std::hash::Hash;

pub trait Hashy<K, V>: Default {
    fn contains_key(&self, k: &K) -> bool;
    fn get(&self, k: &K) -> Option<&V>;
    fn get_mut(&mut self, k: &K) -> Option<&mut V>;
    fn get_unwrapped(&self, k: &K) -> &V;
    fn get_mut_unwrapped(&mut self, k: &K) -> &mut V;
    fn insert(&mut self, k: K, v: V);
    // Iterator -- currently used for debugging purposes only,
    // so avoid having to do too much lifetime hacking we use
    // a Box dyn trait object.
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (K, &'a V)> + 'a>;
}

impl<K: Clone + Hash + Eq, V> Hashy<K, V> for HashMap<K, V> {
    fn contains_key(&self, k: &K) -> bool {
        Self::contains_key(self, k)
    }
    fn get(&self, k: &K) -> Option<&V> {
        HashMap::get(self, k)
    }
    fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        HashMap::get_mut(self, k)
    }
    fn get_unwrapped(&self, k: &K) -> &V {
        self.get(k).unwrap()
    }
    fn get_mut_unwrapped(&mut self, k: &K) -> &mut V {
        self.get_mut(k).unwrap()
    }
    fn insert(&mut self, k: K, v: V) {
        HashMap::insert(self, k, v);
    }
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (K, &'a V)> + 'a> {
        Box::new(self.iter().map(|(k, v)| (k.clone(), v)))
    }
}

// Vector based hashmap
impl<V: Clone> Hashy<usize, V> for Vec<Option<V>> {
    fn contains_key(&self, &k: &usize) -> bool {
        k < self.len() && self[k].is_some()
    }
    fn get(&self, &k: &usize) -> Option<&V> {
        if k < self.len() {
            self[k].as_ref()
        } else {
            None
        }
    }
    fn get_mut(&mut self, &k: &usize) -> Option<&mut V> {
        if k < self.len() {
            self[k].as_mut()
        } else {
            None
        }
    }
    fn get_unwrapped(&self, &k: &usize) -> &V {
        self[k].as_ref().unwrap()
    }
    fn get_mut_unwrapped(&mut self, &k: &usize) -> &mut V {
        self[k].as_mut().unwrap()
    }
    fn insert(&mut self, k: usize, v: V) {
        self.resize(k + 1, None);
        self[k] = Some(v);
    }
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (usize, &'a V)> + 'a> {
        let result = self
            .as_slice()
            .iter()
            .enumerate()
            .map(|(i, v)| Some(i).zip(v.as_ref()))
            .flatten();
        Box::new(result)
    }
}
