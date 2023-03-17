use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VmhdBox {
    pub version: u8,
    pub flags: u32,
    pub graphics_mode: u16,
    pub op_color: RgbColor,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct RgbColor {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl Ibox for VmhdBox {
    fn typ(&self) -> BoxType {
        BoxType::Vmhd
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        8
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!(
            "graphics_mode={} op_color={}{}{}",
            self.graphics_mode, self.op_color.red, self.op_color.green, self.op_color.blue
        );
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for VmhdBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        let graphics_mode = reader.read_u16::<BigEndian>()?;
        let op_color = RgbColor {
            red: reader.read_u16::<BigEndian>()?,
            green: reader.read_u16::<BigEndian>()?,
            blue: reader.read_u16::<BigEndian>()?,
        };

        Ok(VmhdBox {
            version,
            flags,
            graphics_mode,
            op_color,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for VmhdBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
