use super::IntMap;
use std::{collections::HashMap};

pub struct DenseIntMap<V: Clone> {
    inner: Vec<Option<V>>,
    counter: usize,
}

impl <V: Clone> DenseIntMap<V> {
    fn set(&mut self, index: usize, elem: Option<V>) {
        if index >= self.inner.len()
            { self.inner.resize(index,None); }
        self.inner[index] = elem;
    }
}

impl <V: Clone> IntMap<V> for DenseIntMap<V> {
    fn add(&mut self, v: V) -> usize {
        let id = self.counter;
        self.inner.insert(id, Some(v));
        self.counter += 1;
        return id;
    }

    fn put(&mut self, k: usize, elem: Option<V>) {
        if let Some(v) = elem {
            self.set(k, Some(v));
            if k >= self.counter
                { self.counter = k+1 };
        } else { self.set(k, None); }
    }

    fn get(&self, k: usize) -> Option<V> {
        match self.inner.get(k) {
            Some(v) => v.clone(),
            None => None,
        }
    }

    fn get_flattening(&self) -> Result<(Self,HashMap<usize,usize>),()> where Self: Sized {
        // Initialise mapping and flattened vector
        let mut flattened = Vec::new();
        let mut mapping = HashMap::new();
        // Add only elements which are Some(value)
        self.inner.iter().enumerate()
            // Remove all None elements
            .filter_map(|(n,v)|
                if let Some(uv) = v {Some((n,uv))}
                else { None }
            )
            // Add to the mapping and the flattened list
            .for_each(|(n,v)| {
                mapping.insert(n,flattened.len());
                flattened.push(v);
            });
        // Return the flattened list and mapping
        Ok((Self {
                inner: flattened
                    .iter()
                    .map(|x| Some((*x).clone()))
                    .collect(),
                counter: flattened.len()
            }, mapping
        ))
    }
}

impl <V: Clone> Default for DenseIntMap<V> {
    fn default() -> Self {
        Self { inner: Default::default(), counter: Default::default() }
    }
}