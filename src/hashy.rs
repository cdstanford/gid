/*
    Trait for hashmap-like collections.
*/

use std::collections::HashMap;
use std::hash::Hash;

pub trait Hashy<K, V> {
    fn contains_key(&self, k: &K) -> bool;
    fn get(&self, k: &K) -> Option<&V>;
    fn get_mut(&mut self, k: &K) -> Option<&mut V>;
    fn insert(&mut self, k: K, v: V);
}

impl<K: Hash + Eq, V> Hashy<K, V> for HashMap<K, V> {
    fn contains_key(&self, k: &K) -> bool {
        Self::contains_key(self, k)
    }
    fn get(&self, k: &K) -> Option<&V> {
        HashMap::get(self, k)
    }
    fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        HashMap::get_mut(self, k)
    }
    fn insert(&mut self, k: K, v: V) {
        HashMap::insert(self, k, v);
    }
}
