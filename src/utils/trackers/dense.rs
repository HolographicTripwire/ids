use std::{sync::{Mutex, Arc}, collections::HashMap};

use crate::{Identifier, IdentifiedBy, intmaps::DenseIntMap, UpdatableIdStore};

use super::{inner::{IdTrackerInner}, IdTracker};

struct DenseIdTracker<I: Identifier, T: IdentifiedBy<I>> {
    inner: IdTrackerInner<I,T,DenseIntMap<Arc<Mutex<T>>>>
}

impl <I: Identifier, T: IdentifiedBy<I>> IdTracker<I,T> for DenseIdTracker<I,T> {
    fn get(&self, id: I) -> Option<Arc<Mutex<T>>> { self.inner.get(id) }
    fn put(&mut self, element: T) -> Arc<Mutex<T>> { self.inner.put(element) }
    fn flatten(&mut self) -> Result<std::collections::HashMap<I,I>,()> { self.inner.flatten() }
    fn flatten_with<Itr: Iterator<Item = Box<dyn UpdatableIdStore<I>>>>(&mut self, stores_to_update: Itr) -> Result<HashMap<I,I>,()>
        { self.inner.flatten_with(stores_to_update) }
}
