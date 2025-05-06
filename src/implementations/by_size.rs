use std::num::TryFromIntError;

use crate::base::IdImpl;

const TRY_FROM_ERR: &str = "The err branch of an IdImpl's TryFrom was executed. The rust analyser claims that this result is impossible.";

/// An 8-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Id8(u8);
impl IdImpl for Id8 { 
    fn first() -> Self { Self(0) }
    fn next(&self) -> Result<Self,()> {
        if self.0 == u8::MAX || Ok(self.0) == usize::MAX.try_into()
            { Err(()) }
        else { Ok(Self(self.0 + 1)) }
    }
}
impl TryFrom<usize> for Id8 {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match u8::try_from(value) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(err),
        }
    }
}
impl TryFrom<Id8> for usize {
    type Error = TryFromIntError;
    fn try_from(value: Id8) -> Result<Self, Self::Error> {
        match usize::try_from(value.0) {
            Ok(val) => Ok(val),
            Err(_e) => { panic!("{}", TRY_FROM_ERR) },
        }
    }
}

/// A 16-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Id16(u16);
impl IdImpl for Id16 { 
    fn first() -> Self { Self(0) }
    fn next(&self) -> Result<Self,()> {
        if self.0 == u16::MAX || Ok(self.0) == usize::MAX.try_into()
            { Err(()) }
        else { Ok(Self(self.0 + 1)) }
    }
}
impl TryFrom<usize> for Id16 {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match u16::try_from(value) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(err),
        }
    }
}
impl TryFrom<Id16> for usize {
    type Error = TryFromIntError;
    fn try_from(value: Id16) -> Result<Self, Self::Error> {
        match usize::try_from(value.0) {
            Ok(val) => Ok(val),
            Err(_e) => { panic!("{}", TRY_FROM_ERR) },
        }
    }
}

/// A 32-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Id32(u32);
impl IdImpl for Id32 { 
    fn first() -> Self { Self(0) }
    fn next(&self) -> Result<Self,()> {
        if self.0 == u32::MAX || Ok(self.0) == usize::MAX.try_into()
            { Err(()) }
        else { Ok(Self(self.0 + 1)) }
    }
}
impl TryFrom<usize> for Id32 {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match u32::try_from(value) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(err),
        }
    }
}
impl TryFrom<Id32> for usize {
    type Error = TryFromIntError;
    fn try_from(value: Id32) -> Result<Self, Self::Error> {
        match usize::try_from(value.0) {
            Ok(val) => Ok(val),
            Err(_e) => { panic!("{}", TRY_FROM_ERR) },
        }
    }
}

/// An 64-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Id64(u64);
impl IdImpl for Id64 { 
    fn first() -> Self { Self(0) }
    fn next(&self) -> Result<Self,()> {
        if self.0 == u64::MAX || Ok(self.0) == usize::MAX.try_into()
            { Err(()) }
        else { Ok(Self(self.0 + 1)) }
    }
}
impl TryFrom<usize> for Id64 {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match u64::try_from(value) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(err),
        }
    }
}
impl TryFrom<Id64> for usize {
    type Error = TryFromIntError;
    fn try_from(value: Id64) -> Result<Self, Self::Error> {
        match usize::try_from(value.0) {
            Ok(val) => Ok(val),
            Err(_e) => { panic!("{}", TRY_FROM_ERR) },
        }
    }
}

/// An 128-bit [IdImpl]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Id128(u128);
impl IdImpl for Id128 {
    fn first() -> Self { Self(0) }
    fn next(&self) -> Result<Self,()> {
        if self.0 == u128::MAX || Ok(self.0) == usize::MAX.try_into()
            { Err(()) }
        else { Ok(Self(self.0 + 1)) }
    }
}
impl TryFrom<usize> for Id128 {
    type Error = TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match u128::try_from(value) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(err),
        }
    }
}
impl TryFrom<Id128> for usize {
    type Error = TryFromIntError;
    fn try_from(value: Id128) -> Result<Self, Self::Error> {
        match usize::try_from(value.0) {
            Ok(val) => Ok(val),
            Err(_e) => { panic!("{}", TRY_FROM_ERR) },
        }
    }
}
