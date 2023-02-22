use std::collections::HashMap;

use bimap::BiHashMap;

use crate::Identifier;

use super::UpdatableIdStore;

pub struct IdLinker<T1: Identifier, T2: Identifier> {
    map: BiHashMap<T1,T2>
}

impl <T1: Identifier, T2: Identifier> IdLinker<T1,T2> {
    pub fn get_by_left(&self, left: T1) -> Option<&T2>
        { self.map.get_by_left(&left) }
    pub fn get_by_right(&self, right: T2) -> Option<&T1>
        { self.map.get_by_right(&right) }
    
    pub fn put(&mut self, left: Option<T1>, right: Option<T2>) {
        match (left, right) {
            (None, None) => {},
            (None, Some(right)) => self.delete_by_right(&right),
            (Some(left), None) =>  self.delete_by_left (&left ),
            (Some(left), Some(right)) => self.insert(left, right),
        }
    }

    pub fn insert(&mut self, left: T1, right: T2)
        { self.map.insert(left, right); }
    pub fn delete_by_left(&mut self, left: &T1)
        { self.map.remove_by_left(left); }
    pub fn delete_by_right(&mut self, right: &T2)
        { self.map.remove_by_right(right); }

    pub fn left_updater(&mut self) -> LeftLinkerUpdater<T1,T2>
        { LeftLinkerUpdater { linker: self } }
    pub fn right_updater(&mut self) -> RightLinkerUpdater<T1,T2>
        { RightLinkerUpdater { linker: self } }
}

/// A struct which provides an interface for updating IdLinkers by their left id type
pub struct LeftLinkerUpdater<'a, T1: Identifier, T2: Identifier> {
    linker: &'a mut IdLinker<T1,T2>
}

impl <T1: Identifier, T2: Identifier> UpdatableIdStore<T1> for LeftLinkerUpdater<'_,T1,T2> {
    fn update_ids(&mut self, mapping: &HashMap<T1,T1>) {
        let mut new_map = BiHashMap::<T1,T2>::new();
        for (left,right) in &self.linker.map {
            let new_left = match mapping.get(&left) {
                Some(val) => val,
                None => panic!("Attempted to update an UpdatableIdStore without supplying a complete list of updates"),
            }; new_map.insert(*new_left, *right);
        }
        self.linker.map = new_map;
    }
}

/// A struct which provides an interface for updating IdLinkers by their left id type
pub struct RightLinkerUpdater<'a, T1: Identifier, T2: Identifier> {
    linker: &'a mut IdLinker<T1,T2>
}

impl <T1: Identifier, T2: Identifier> UpdatableIdStore<T2> for RightLinkerUpdater<'_,T1,T2> {
    fn update_ids(&mut self, mapping: &HashMap<T2,T2>) {
        let mut new_map = BiHashMap::<T1,T2>::new();
        for (left,right) in &self.linker.map {
            let new_right = match mapping.get(&right) {
                Some(val) => val,
                None => panic!("Attempted to update an UpdatableIdStore without supplying a complete list of updates"),
            }; new_map.insert(*left, *new_right);
        }
        self.linker.map = new_map;
    }
}