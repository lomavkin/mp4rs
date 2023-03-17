use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;
use crate::types::FixedPointI8;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SmhdBox {
    pub version: u8,
    pub flags: u32,
    pub balance: FixedPointI8,
}

impl Ibox for SmhdBox {
    fn typ(&self) -> BoxType {
        BoxType::Smhd
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        4
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("balance={}", self.balance.value());
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for SmhdBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        let balance = FixedPointI8::new_raw(reader.read_i16::<BigEndian>()?);

        Ok(SmhdBox {
            version,
            flags,
            balance,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for SmhdBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
