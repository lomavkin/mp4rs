use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DinfBox;

impl Ibox for DinfBox {
    fn typ(&self) -> BoxType {
        BoxType::Dinf
    }

    fn data_size(&self) -> u64 {
        0
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        Ok(String::from(""))
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for DinfBox {
    fn read(_: &mut R, _: &BoxHeader) -> Result<Self> {
        Ok(DinfBox)
    }
}

impl<W: Write> WriteBox<&mut W> for DinfBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
