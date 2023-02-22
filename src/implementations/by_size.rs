use crate::base::IdImpl;

/// An 8-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id8 { val: u8 }
impl IdImpl for Id8 { 
    fn first() -> Self { Self { val: 0 } }
    fn next(&self) -> Result<Self,()> {
        if self.val == u8::MAX { Err(()) }
        else { Ok(Self { val: self.val + 1 }) }
    }
}

/// A 16-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id16 { val: u16 }
impl IdImpl for Id16 { 
    fn first() -> Self { Self { val: 0 } }
    fn next(&self) -> Result<Self,()> {
        if self.val == u16::MAX { Err(()) }
        else { Ok(Self { val: self.val + 1 }) }
    }
}

/// A 32-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id32 { val: u32 }
impl IdImpl for Id32 { 
    fn first() -> Self { Self { val: 0 } }
    fn next(&self) -> Result<Self,()> {
        if self.val == u32::MAX { Err(()) }
        else { Ok(Self { val: self.val + 1 }) }
    }
}

/// An 64-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id64 { val: u64 }
impl IdImpl for Id64 { 
    fn first() -> Self { Self { val: 0 } }
    fn next(&self) -> Result<Self,()> {
        if self.val == u64::MAX { Err(()) }
        else { Ok(Self { val: self.val + 1 }) }
    }
}

/// An 128-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id128 { val: u128 }
impl IdImpl for Id128 { 
    fn first() -> Self { Self { val: 0 } }
    fn next(&self) -> Result<Self,()> {
        if self.val == u128::MAX { Err(()) }
        else { Ok(Self { val: self.val + 1 }) }
    }
}