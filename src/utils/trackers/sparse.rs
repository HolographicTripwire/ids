use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{Identifier, IdentifiedBy};

//use crate::utils::{trackers::IdTracker, counters::IdCounter};

/*
pub struct SparseIdTracker<I: Identifier, T: IdentifiedBy<I>> {
    objects: HashMap<I,Arc<Mutex<T>>>,
    id_counter: IdCounter<I>,
}

impl <I: Identifier, T: IdentifiedBy<I>> SparseIdTracker<I, T> {
    
}
*/