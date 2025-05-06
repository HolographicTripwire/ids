use std::marker::PhantomData;
use std::num::TryFromIntError;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use crate::intmaps::IntMap;
use crate::{IdentifiedBy, Identifier, UpdatableIdStore};

use super::IdTracker;

const RETRIEVE_NEW_ELEMENT_ERROR: &str = "Ids: Failed to retrieve an element which had just been inserted";
const CONVERT_FROM_USIZE_ERROR: &str = "Ids: failed to convert from usize";
const CONVERT_TO_USIZE_ERROR: &str = "Ids: failed to convert to usize";

pub struct IdTrackerInner<I: Identifier,T: IdentifiedBy<I>, M: IntMap<Arc<Mutex<T>>>> {
    p: PhantomData<(I,T)>,
    map: M
}

impl <I: Identifier, T: IdentifiedBy<I>, M: IntMap<Arc<Mutex<T>>>> IdTrackerInner<I,T,M> {
    /**
    Insert a reference-counted object into the tracker.

    Note that this will update the id of the object. 
    The modified object will be returned, if the mutex is unpoisoned.
    Otherwise an [Err] will be returned
     */
    fn insert<'a>(&mut self, element: Arc<Mutex<T>>) -> Result<Arc<Mutex<T>>,()> {
        // Get the id to use
        let id = self.map.add(element);

        // Get the element and its id
        let element = match self.map.get(id) {
            Some(elem) => elem,
            None => panic!("{}",RETRIEVE_NEW_ELEMENT_ERROR),
        }; let id = I::try_from(id).expect(CONVERT_FROM_USIZE_ERROR);
        
        // Set the element's id and return it
        match element.lock() {
            Ok(mut guard) => guard.set_id(id),
            Err(_) => return Err(())
        }; return Ok(element);
    }
}

impl <I: Identifier, T: IdentifiedBy<I>, M: IntMap<Arc<Mutex<T>>>> IdTracker<I,T> for IdTrackerInner<I,T,M> {
    fn get(&self, id: I) -> Option<Arc<Mutex<T>>> {
        self.map.get(id.try_into().expect(CONVERT_TO_USIZE_ERROR))
    }
    
    fn put(&mut self, element: T) -> Arc<Mutex<T>> {
        let result = self.insert(Arc::new(Mutex::new(element)));
        result.expect("Ids: A brand new mutex was poisoned")
    }
    
    /**
    Flatten this tracker, collapsing all spaces where elements have been deleted.

    If one of the elements' mutexes is poisoned, returns the corresponding [PoisonError]
    Otherwise, returns the mappings from old ids to new ones.
     */
    fn flatten(&mut self) -> Result<HashMap<I,I>,()> {
        let (flattened_map,mapping) = match self.map.get_flattening() {
            Ok(vals) => vals,
            Err(err) => return Err(err),
        };
        self.map = flattened_map;
        let id_mapping = map_to_ids(mapping);
        return Ok(id_mapping.expect(CONVERT_FROM_USIZE_ERROR));
    }

    /**
    If one of the elements' mutexes is poisoned, returns the corresponding [PoisonError]
    Otherwise, returns the mappings from old ids to new ones.
     */
    fn flatten_with<Itr: Iterator<Item = Box<dyn UpdatableIdStore<I>>>>(&mut self, stores_to_update: Itr) -> Result<HashMap<I,I>,()> {
        // Flatten self first, returning any poison errors that occur
        let map = match self.flatten() {
            Ok(vals) => vals,
            Err(err) => return Err(err),
        }; // Then carry the changes forward to all provided stores
        for mut store in stores_to_update.into_iter()
            { store.update_ids(&map) }
        return Ok(map);
    }
}

fn map_to_ids<I: Identifier>(usize_map: HashMap<usize,usize>) -> Result<HashMap<I,I>,TryFromIntError> {
    let mut id_map = HashMap::<I,I>::new();
    for (k_old,v_old) in usize_map {
        match (k_old.try_into(), v_old.try_into()) {
            (Ok(k),Ok(v)) => { id_map.insert(k, v); },
            (Ok(_), Err(e)) => return Err(e),
            (Err(e), Ok(_)) => return Err(e),
            (Err(e), Err(_)) => return Err(e),
        }
    }; return Ok(id_map);
}
