use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StszBox {
    pub version: u8,
    pub flags: u32,
    pub sample_size: u32,
    pub sample_count: u32,
    pub sample_sizes: Vec<u32>,
}

impl Ibox for StszBox {
    fn typ(&self) -> BoxType {
        BoxType::Stsz
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        8 + (4 * self.sample_sizes.len()) as u64
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("sample_sizes={}", self.sample_sizes.len());
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for StszBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        let sample_size = reader.read_u32::<BigEndian>()?;
        let stsz_item_size = if sample_size == 0 {
            std::mem::size_of::<u32>() // entry_size
        } else {
            0
        };
        let sample_count = reader.read_u32::<BigEndian>()?;
        let mut sample_sizes = Vec::new();
        if sample_size == 0 {
            sample_sizes.reserve(sample_count as usize);
            for _ in 0..sample_count {
                let sample_number = reader.read_u32::<BigEndian>()?;
                sample_sizes.push(sample_number);
            }
        }

        Ok(StszBox {
            version,
            flags,
            sample_size,
            sample_count,
            sample_sizes,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for StszBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
