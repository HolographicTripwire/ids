mod sparse;
mod dense;

use std::{collections::HashMap};
pub use dense::DenseIntMap;
pub use sparse::SparseIntMap;

pub trait IntMap<V> {
    fn add(&mut self, elem: V) -> usize;
    fn rmv(&mut self, k: usize) { self.put(k,None); }
    fn put(&mut self, k: usize, elem: Option<V>);
    fn get(&self, k: usize) -> Option<V>;

    /**
    Get what the result would be if this tracker were flattened (see the flatten() operation)

    If one of the mutexes is poisoned, return an [Err]
    Otherwise, return the updated self, as well as the mappings from old to new ids.
     */
    fn get_flattening(&self) -> Result<(Self,HashMap<usize,usize>),()> where Self: Sized;
}