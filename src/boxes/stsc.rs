use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StscBox {
    pub version: u8,
    pub flags: u32,
    pub entries: Vec<StscEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StscEntry {
    pub first_chunk: u32,
    pub sample_per_chunk: u32,
    pub sample_description_index: u32,
    pub first_sample: u32,
}

impl Ibox for StscBox {
    fn typ(&self) -> BoxType {
        BoxType::Stsc
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        4 + (12 * self.entries.len()) as u64
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("entries={}", self.entries.len());
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for StscBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        let entry_count = reader.read_u32::<BigEndian>()?;
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            let entry = StscEntry {
                first_chunk: reader.read_u32::<BigEndian>()?,
                sample_per_chunk: reader.read_u32::<BigEndian>()?,
                sample_description_index: reader.read_u32::<BigEndian>()?,
                first_sample: 0,
            };
            entries.push(entry);
        }

        let mut sample_id = 1;
        for i in 0..entry_count {
            let (first_chunk, sample_per_chunk) = {
                let entry = entries.get_mut(i as usize).unwrap();
                entry.first_sample = sample_id;
                (entry.first_chunk, entry.sample_per_chunk)
            };
            if i < entry_count - 1 {
                let next_entry = entries.get(i as usize + 1).unwrap();
                sample_id = next_entry
                    .first_chunk
                    .checked_sub(first_chunk)
                    .and_then(|n| n.checked_mul(sample_per_chunk))
                    .and_then(|n| n.checked_add(sample_id))
                    .ok_or(Error::InvalidData(
                        "attempt to calculate stsc sample_id with overflow",
                    ))?;
            }
        }

        Ok(StscBox {
            version,
            flags,
            entries,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for StscBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
