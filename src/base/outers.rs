use std::hash::Hash;
use std::fmt::Debug;
use std::num::TryFromIntError;

/**
An identifier where the next value can be retrieved without a result.

This trait should be implemented once for every [IdentifiedBy] implementation
 */
pub trait Identifier: Clone + Copy + PartialEq + Eq + Hash + Debug
    + TryFrom<usize,Error = TryFromIntError> + TryInto<usize,Error = TryFromIntError> {
    /// Get the first id of this type
    fn first() -> Self;
    /// Get the next id, after this one
    fn next(self) -> Self;
}

/**
An object which can be identified purely from an [Identifier]

This trait should be implemented once for every [Identifier] implementation
 */
pub trait IdentifiedBy<T: Identifier> {
    ///Get the currently stored id.
    fn get_id(&self) -> T;
    /**
    Set the id of this object.

    Note that this method should, for most use cases, never be called by anything outside of this module.
    It is mostly for usage by internal structs, such as the [crate::IdTracker]
     */
    fn set_id(&mut self,id: T);
}

impl <T: Identifier> Eq for dyn IdentifiedBy<T> {}
impl <T: Identifier> PartialEq for dyn IdentifiedBy<T> {
    fn eq(&self, other: &Self) -> bool
        { self.get_id() == other.get_id() }
}
impl <T: Identifier> Hash for dyn IdentifiedBy<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
        { self.get_id().hash(state) }
}
