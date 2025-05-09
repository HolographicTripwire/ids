pub mod trackers;
pub mod linkers;

use std::collections::HashMap;

use crate::Identifier;

/**
For structs which store ids, and can be updated
 */
pub trait UpdatableIdStore<T: Identifier> {
    /// Replace all ids in this struct with some corresponding id
    fn update_ids(&mut self, mapping: &HashMap<T,T>);
}