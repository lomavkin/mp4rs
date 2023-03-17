use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PaspBox {
    h_spacing: u32,
    v_spacing: u32,
}

impl Ibox for PaspBox {
    fn typ(&self) -> BoxType {
        BoxType::Pasp
    }

    fn data_size(&self) -> u64 {
        8
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("h_spacing={} v_spacing={}", self.h_spacing, self.v_spacing,);
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for PaspBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let h_spacing = reader.read_u32::<BigEndian>()?;
        let v_spacing = reader.read_u32::<BigEndian>()?;

        Ok(PaspBox {
            h_spacing,
            v_spacing,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for PaspBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
