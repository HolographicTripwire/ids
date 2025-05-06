mod inner;
mod dense;
mod sparse;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};

pub use dense::DenseIdTracker;

use crate::{IdentifiedBy, Identifier, UpdatableIdStore};

pub trait IdTracker<I: Identifier,T: IdentifiedBy<I>> {
    
    fn get(&self, id: I) -> Option<Arc<Mutex<T>>>;
    fn put(&mut self, element: T) -> Arc<Mutex<T>>;
    
    /**
    Flatten this tracker, collapsing all spaces where elements have been deleted.

    If one of the elements' mutexes is poisoned, returns the corresponding [PoisonError]
    Otherwise, returns the mappings from old ids to new ones.
     */
    fn flatten(&mut self) -> Result<HashMap<I,I>,()>;

    /**
    If one of the elements' mutexes is poisoned, returns the corresponding [PoisonError]
    Otherwise, returns the mappings from old ids to new ones.
     */
    fn flatten_with<Itr: Iterator<Item = Box<dyn UpdatableIdStore<I>>>>(&mut self, stores_to_update: Itr) -> Result<HashMap<I,I>,()>;
}
