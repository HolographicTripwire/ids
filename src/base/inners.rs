use std::hash::Hash;
use std::num::TryFromIntError;

/**
[IdInner] is a type which wraps an [IdImpl] and provides it with an error message.

This should only be used as part of [Id] objects.
 */
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct IdInner<'a, T: IdImpl> {
    inner_type: T,
    out_of_ids_msg: &'a str,
}

impl <'a, T: IdImpl> IdInner<'a, T> {
    /// Get the first id, and give it an error message for all future ids to use
    pub fn first(out_of_ids_msg: &'a str) -> Self {
        Self { inner_type: T::first(), out_of_ids_msg }
    }

    /// Get the next id after this one, panicking if we are out of ids
    pub fn next(&self) -> Self {
        match self.inner_type.next() {
            Ok(next) => Self { 
                inner_type: next,
                out_of_ids_msg: self.out_of_ids_msg
            }, Err(()) => panic!("{}", self.out_of_ids_msg)
        }
    }
}

impl <'a, T: IdImpl> TryInto<usize> for IdInner<'a,T> {
    type Error = ();
    fn try_into(self) -> Result<usize, Self::Error> {
        match self.inner_type.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(()),
        }
    }
}

impl <'a, T: IdImpl> From<usize> for IdInner<'a,T> {
    fn from(val: usize) -> Self { val.into() }
}

pub trait IdImpl: Clone + Copy + PartialEq + Eq + Hash + 
    TryFrom<usize, Error = TryFromIntError> + TryInto<usize, Error = TryFromIntError> {
    /// Get the first id of this type
    fn first() -> Self;
    /// Get the next id, after this one
    fn next(&self) -> Result<Self,()>;
}
