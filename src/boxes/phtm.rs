use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PhtmBox;

impl Ibox for PhtmBox {
    fn typ(&self) -> BoxType {
        BoxType::Phtm
    }

    fn data_size(&self) -> u64 {
        0
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        Ok("root".to_string())
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for PhtmBox {
    fn read(_: &mut R, _: &BoxHeader) -> Result<Self> {
        Ok(PhtmBox)
    }
}

impl<W: Write> WriteBox<&mut W> for PhtmBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        Ok(0)
    }
}
