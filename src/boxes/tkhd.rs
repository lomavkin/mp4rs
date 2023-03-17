use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;
use crate::types::{FixedPointU16, FixedPointU8};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Matrix {
    pub a: i32,
    pub b: i32,
    pub u: i32,
    pub c: i32,
    pub d: i32,
    pub v: i32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#x} {:#x} {:#x} {:#x} {:#x} {:#x} {:#x} {:#x} {:#x}",
            self.a, self.b, self.u, self.c, self.d, self.v, self.x, self.y, self.w
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TkhdBox {
    pub version: u8,
    pub flags: u32,
    pub creation_time: u64,
    pub modification_time: u64,
    pub track_id: u32,
    pub duration: u64,
    pub layer: u16,
    pub alternate_group: u16,
    pub volume: FixedPointU8,
    pub matrix: Matrix,
    pub width: FixedPointU16,
    pub height: FixedPointU16,
}

impl Ibox for TkhdBox {
    fn typ(&self) -> BoxType {
        BoxType::Tkhd
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        let mut size = if self.version == 1 { 32 } else { 20 };
        size += 60;
        size
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!(
            "creation_time={} track_id={} duration={} layer={} volume={} matrix={} width={} height={}",
            self.creation_time,
            self.track_id,
            self.duration,
            self.layer,
            self.volume.value(),
            self.matrix,
            self.width.value(),
            self.height.value()
        );
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for TkhdBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;
        let (creation_time, modification_time, track_id, _, duration) = if version == 1 {
            (
                reader.read_u64::<BigEndian>()?,
                reader.read_u64::<BigEndian>()?,
                reader.read_u32::<BigEndian>()?,
                reader.read_u32::<BigEndian>()?,
                reader.read_u64::<BigEndian>()?,
            )
        } else if version == 0 {
            (
                reader.read_u32::<BigEndian>()? as u64,
                reader.read_u32::<BigEndian>()? as u64,
                reader.read_u32::<BigEndian>()?,
                reader.read_u32::<BigEndian>()?,
                reader.read_u32::<BigEndian>()? as u64,
            )
        } else {
            return Err(Error::InvalidData("version must be 0 or 1"));
        };

        reader.read_u64::<BigEndian>()?; // reserved
        let layer = reader.read_u16::<BigEndian>()?;
        let alternate_group = reader.read_u16::<BigEndian>()?;
        let volume = FixedPointU8::new_raw(reader.read_u16::<BigEndian>()?);

        reader.read_u16::<BigEndian>()?; // reserved
        let matrix = Matrix {
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

        let width = FixedPointU16::new_raw(reader.read_u32::<BigEndian>()?);
        let height = FixedPointU16::new_raw(reader.read_u32::<BigEndian>()?);

        Ok(TkhdBox {
            version,
            flags,
            creation_time,
            modification_time,
            track_id,
            duration,
            layer,
            alternate_group,
            volume,
            matrix,
            width,
            height,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for TkhdBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
