use std::{hash::Hash, collections::HashMap};

use super::IntMap;

struct SparseIntMap<K: Hash> {
    inner: HashMap<usize,K>,
    counter: usize,
}

impl <V: Hash> SparseIntMap<V> {
    fn set(&mut self, k: usize, elem: Option<V>) {
        match elem {
            Some(v) => { self.inner.insert(k,v); },
            None => { self.inner.remove(&k); },
        }
    }
}

impl <V: Hash + Clone> IntMap<V> for SparseIntMap<V> {
    fn add(&mut self, v: V) -> usize {
        let id = self.counter;
        self.set(id, Some(v));
        self.counter += 1;
        return id;
    }

    fn put(&mut self, k: usize, elem: Option<V>) {
        if let Some(v) = elem {
            self.set(k,Some(v));
            if k >= self.counter
                { self.counter = k+1 };
        } else { self.set(k, None); }
    }

    fn get(&self, k: usize) -> Option<V> {
        match self.inner.get(&k) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    fn get_flattening(&self) -> Result<(Self,HashMap<usize,usize>),()> where Self: Sized {
        // Initialise mapping and flattened vector
        let mut flattened = HashMap::new();
        let mut mapping = HashMap::new();
        // Add only elements which are Some(value)
        self.inner.iter().for_each(|(n,v)| {
            mapping.insert(*n, flattened.len());
            flattened.insert(flattened.len(), (*v).clone());
        });
        // Return the flattened list and mapping
        Ok((Self {
            counter: flattened.len(),
            inner: flattened,
        }, mapping
        ))
    }
}

impl <K: Hash> Default for SparseIntMap<K> {
    fn default() -> Self {
        Self { inner: HashMap::default(), counter: 0 }
    }
}
