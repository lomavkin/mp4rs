use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;
use crate::types::FourCC;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HdlrBox {
    pub version: u8,
    pub flags: u32,
    pub handler_type: FourCC,
    pub name: String,
}

impl Ibox for HdlrBox {
    fn typ(&self) -> BoxType {
        BoxType::Hdlr
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        // pre_defined(4) + handler_type(4) + reserved(4*3) + name.len() + null-terminated(1)
        20 + self.name.len() as u64 + 1
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("handler_type={} name={}", self.handler_type, self.name);
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for HdlrBox {
    fn read(reader: &mut R, header: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        reader.read_u32::<BigEndian>()?; // pre-defined
        let handler = reader.read_u32::<BigEndian>()?;

        super::rel_skip(reader, 12)?; // reserved

        let buf_size = header
            .size
            .checked_sub(8 + 4 + 20 + 1)
            .ok_or(Error::InvalidData("hdlr size too small"))?;
        let mut buf = vec![0u8; buf_size as usize];
        reader.read_exact(&mut buf)?;

        let handler_string = match String::from_utf8(buf) {
            Ok(t) => {
                if t.len() != buf_size as usize {
                    return Err(Error::InvalidData("string too small"));
                }
                t
            }
            _ => String::from("null"),
        };

        Ok(HdlrBox {
            version,
            flags,
            handler_type: From::from(handler),
            name: handler_string,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for HdlrBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
