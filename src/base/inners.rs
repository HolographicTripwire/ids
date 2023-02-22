use std::hash::Hash;

/**
[IdInner] is a type which wraps an [IdImpl] and provides it with an error message.

This should only be used as part of [Id] objects.
 */
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
            },
            Err(()) => panic!("{}", self.out_of_ids_msg)
        }
    }
}

pub trait IdImpl: Clone + Copy + PartialEq + Eq + Hash {
    /// Get the first id of this type
    fn first() -> Self;
    /// Get the next id, after this one
    fn next(&self) -> Result<Self,()>;
}
