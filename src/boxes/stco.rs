use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StcoBox {
    pub version: u8,
    pub flags: u32,
    pub chunk_offsets: Vec<i32>,
}

impl Ibox for StcoBox {
    fn typ(&self) -> BoxType {
        BoxType::Stco
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        4 + (4 * self.chunk_offsets.len()) as u64
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("chunk_offsets={}", self.chunk_offsets.len());
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for StcoBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        let entry_count = reader.read_u32::<BigEndian>()?;
        let mut chunk_offsets = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            let chunk_offset = reader.read_i32::<BigEndian>()?;
            chunk_offsets.push(chunk_offset);
        }

        Ok(StcoBox {
            version,
            flags,
            chunk_offsets,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for StcoBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
