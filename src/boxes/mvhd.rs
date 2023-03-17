use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{tkhd, BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;
use crate::types::{FixedPointU16, FixedPointU8};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MvhdBox {
    pub version: u8,
    pub flags: u32,
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub rate: FixedPointU16,
    pub volume: FixedPointU8,
    pub matrix: tkhd::Matrix,
    pub next_track_id: u32,
}

impl Ibox for MvhdBox {
    fn typ(&self) -> BoxType {
        BoxType::Mvhd
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        let mut size = if self.version == 1 { 28 } else { 16 };
        size += 80;
        size
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!(
            "creation_time={} timescale={} duration={} rate={} volume={}, matrix={}, next_track_id={}",
            self.creation_time,
            self.timescale,
            self.duration,
            self.rate.value(),
            self.volume.value(),
            self.matrix,
            self.next_track_id
        );
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for MvhdBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        let (creation_time, modification_time, timescale, duration) = if version == 1 {
            (
                reader.read_u64::<BigEndian>()?,
                reader.read_u64::<BigEndian>()?,
                reader.read_u32::<BigEndian>()?,
                reader.read_u64::<BigEndian>()?,
            )
        } else if version == 0 {
            (
                reader.read_u32::<BigEndian>()? as u64,
                reader.read_u32::<BigEndian>()? as u64,
                reader.read_u32::<BigEndian>()?,
                reader.read_u32::<BigEndian>()? as u64,
            )
        } else {
            return Err(Error::InvalidData("version must be 0 or 1"));
        };
        let rate = FixedPointU16::new_raw(reader.read_u32::<BigEndian>()?);
        let volume = FixedPointU8::new_raw(reader.read_u16::<BigEndian>()?);

        reader.read_u16::<BigEndian>()?; // reserved = 0
        reader.read_u64::<BigEndian>()?; // reserved = 0

        let matrix = tkhd::Matrix {
            a: reader.read_i32::<BigEndian>()?,
            b: reader.read_i32::<BigEndian>()?,
            u: reader.read_i32::<BigEndian>()?,
            c: reader.read_i32::<BigEndian>()?,
            d: reader.read_i32::<BigEndian>()?,
            v: reader.read_i32::<BigEndian>()?,
            x: reader.read_i32::<BigEndian>()?,
            y: reader.read_i32::<BigEndian>()?,
            w: reader.read_i32::<BigEndian>()?,
        };

        super::rel_skip(reader, 24)?; // pre_defined = 0

        let next_track_id = reader.read_u32::<BigEndian>()?;

        Ok(MvhdBox {
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            rate,
            volume,
            matrix,
            next_track_id,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for MvhdBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
