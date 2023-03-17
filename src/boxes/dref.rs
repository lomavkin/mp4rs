use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DrefBox {
    version: u8,
    flags: u32,
    entry_count: u32,
}

impl Ibox for DrefBox {
    fn typ(&self) -> BoxType {
        BoxType::Dref
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
        let s = format!("entry_count={}", self.entry_count);
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for DrefBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        let entry_count = reader.read_u32::<BigEndian>()?;

        Ok(DrefBox {
            version,
            flags,
            entry_count,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for DrefBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
