use serde::Serialize;
use std::fmt;

use crate::boxes::BoxType;

pub use bytes::Bytes;
pub use num_rational::Ratio;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct FixedPointU8(Ratio<u16>);

impl FixedPointU8 {
    pub fn new(val: u8) -> Self {
        Self(Ratio::new_raw(val as u16 * 0x100, 0x100))
    }

    pub fn new_raw(val: u16) -> Self {
        Self(Ratio::new_raw(val, 0x100))
    }

    pub fn value(&self) -> u8 {
        self.0.to_integer() as u8
    }

    pub fn raw_value(&self) -> u16 {
        *self.0.numer()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct FixedPointI8(Ratio<i16>);

impl FixedPointI8 {
    pub fn new(val: i8) -> Self {
        Self(Ratio::new_raw(val as i16 * 0x100, 0x100))
    }

    pub fn new_raw(val: i16) -> Self {
        Self(Ratio::new_raw(val, 0x100))
    }

    pub fn value(&self) -> i8 {
        self.0.to_integer() as i8
    }

    pub fn raw_value(&self) -> i16 {
        *self.0.numer()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct FixedPointU16(Ratio<u32>);

impl FixedPointU16 {
    pub fn new(val: u16) -> Self {
        Self(Ratio::new_raw(val as u32 * 0x10000, 0x10000))
    }

    pub fn new_raw(val: u32) -> Self {
        Self(Ratio::new_raw(val, 0x10000))
    }

    pub fn value(&self) -> u16 {
        self.0.to_integer() as u16
    }

    pub fn raw_value(&self) -> u32 {
        *self.0.numer()
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct FourCC {
    pub value: [u8; 4],
}

impl fmt::Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code: u32 = self.into();
        let string = String::from_utf8_lossy(&self.value[..]);
        write!(f, "{string} / {code:#010X}")
    }
}

impl fmt::Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.value[..]))
    }
}

impl From<u32> for FourCC {
    fn from(n: u32) -> FourCC {
        FourCC {
            value: n.to_be_bytes(),
        }
    }
}

impl From<FourCC> for u32 {
    fn from(f: FourCC) -> u32 {
        (&f).into()
    }
}

impl From<&FourCC> for u32 {
    fn from(f: &FourCC) -> u32 {
        u32::from_be_bytes(f.value)
    }
}

impl From<[u8; 4]> for FourCC {
    fn from(value: [u8; 4]) -> Self {
        FourCC { value }
    }
}

impl From<BoxType> for FourCC {
    fn from(b: BoxType) -> Self {
        let n: u32 = b.into();
        From::from(n)
    }
}
