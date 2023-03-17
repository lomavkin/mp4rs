use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StssBox {
    pub version: u8,
    pub flags: u32,
    pub entries: Vec<u32>,
}

impl Ibox for StssBox {
    fn typ(&self) -> BoxType {
        BoxType::Stss
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        4 + (4 * self.entries.len()) as u64
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("entries={}", self.entries.len());
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for StssBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        let entry_count = reader.read_u32::<BigEndian>()?;
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            entries.push(reader.read_u32::<BigEndian>()?);
        }

        Ok(StssBox {
            version,
            flags,
            entries,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for StssBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
