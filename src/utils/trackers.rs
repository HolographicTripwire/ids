use std::{collections::HashMap};
use std::sync::{Mutex, Arc};
use std::hash::Hash;

use crate::{IdentifiedBy, Identifier};

use super::UpdatableIdStore;

pub struct IdTracker<I: Identifier, T: IdentifiedBy<I>> {
    objects: HashMap<I,Arc<Mutex<T>>>,
    id_counter: IdCounter<I>,
}

impl <I: Identifier, T: IdentifiedBy<I> + Eq + Hash> IdTracker<I,T> {
    /// Get the object corresponding to a given [Identifier]
    pub fn get(&self, id: I) -> Option<Arc<Mutex<T>>>
        { self.objects.get(&id).cloned() }
    
    /**
    Put a new object into the tracker.

    Note that this will update the id of the object. The modified object will be returned.
     */
    pub fn put(&mut self, mut element: T) -> Arc<Mutex<T>> {
        let new_id = self.id_counter.next();
        element.set_id(new_id);
        let guarded = Arc::new(Mutex::new(element));
        self.objects.insert(new_id, guarded.clone());
        return guarded;
    }
    /**
    Insert a reference-counted object into the tracker.

    Note that this will update the id of the object. 
    The modified object will be returned, if the mutex is unpoisoned.
    Otherwise an [Err] will be returned
     */
    fn insert<'a>(&mut self, element: Arc<Mutex<T>>) -> Result<Arc<Mutex<T>>,()> {
        let mut elem = match element.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(())
        };
        let new_id = self.id_counter.next();
        elem.set_id(new_id);
        self.objects.insert(new_id, element.clone());
        Ok(element.clone())
    }
    
    /**
    Flatten this tracker, collapsing all spaces where elements have been deleted.

    If one of the elements' mutexes is poisoned, returns the corresponding [PoisonError]
    Otherwise, returns the mappings from old ids to new ones.
     */
    pub fn flatten(&mut self) -> Result<HashMap<I,I>,()> {
        let (flattened,mapping) = match self.get_flattening() {
            Ok(vals) => vals,
            Err(err) => return Err(err),
        };
        self.objects = flattened.objects;
        self.id_counter = flattened.id_counter;
        return Ok(mapping);
    }

    /**
    Flatten this tracker, collapsing all spaces where elements have been deleted, and updating the elements in any provided id stores

    If one of the elements' mutexes is poisoned, returns the corresponding [PoisonError]
    Otherwise, returns the mappings from old ids to new ones.
     */
    pub fn flatten_with<Itr: Iterator<Item = Box<dyn UpdatableIdStore<I>>>>(&mut self, stores_to_update: Itr) -> Result<HashMap<I,I>,()> {
        // Flatten self first, returning any poison errors that occur
        let map = match self.flatten() {
            Ok(vals) => vals,
            Err(err) => return Err(err),
        }; // Then carry the changes forward to all provided stores
        for mut store in stores_to_update.into_iter()
            { store.update_ids(&map) }
        return Ok(map);
    }

    /**
    Get what the result would be if this tracker were flattened (see the flatten() operation)

    If one of the mutexes is poisoned, return an [Err]
    Otherwise, return the updated self, as well as the mappings from old to new ids.
     */
    fn get_flattening(&self) -> Result<(Self,HashMap<I,I>),()> {
        let mut flattened = Self::default();
        let mut mapping: HashMap<I,I> = HashMap::new();
        for (old_id, element) in self.objects.iter() {
            let new_id = match flattened.insert(element.clone()) {
                Ok(element) => match element.lock() {
                    Ok(guard) => guard.get_id(),
                    Err(_) => return Err(()),
                },
                Err(err) => return Err(err),
            };
            mapping.insert(*old_id, new_id);
        } return Ok((flattened,mapping));
    }
}

impl <I: Identifier, T: IdentifiedBy<I>> Default for IdTracker<I,T> {
    fn default() -> Self { Self {
        objects: HashMap::default(),
        id_counter: IdCounter::default(),
    }}
}




pub struct IdCounter<T: Identifier> { id: T, }

impl <T: Identifier> IdCounter<T> {
    /// Return the value stored by this counter and increment it to the next value
    pub fn next(&mut self) -> T {
        let return_id = self.id;
        self.id = self.id.next();
        return_id
    }
}

impl <T: Identifier> Default for IdCounter<T> {
    fn default() -> Self { Self { id: Identifier::first() } }
}
