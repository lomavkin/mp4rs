use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UrlBox {
    version: u8,
    flags: u32,
    location: String,
}

impl Ibox for UrlBox {
    fn typ(&self) -> BoxType {
        BoxType::Url
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        if self.flags == 0x1 {
            0
        } else {
            self.location.len() as u64 + 1
        }
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("location={}", self.location);
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for UrlBox {
    fn read(reader: &mut R, header: &BoxHeader) -> Result<Self> {
        let (version, flags) = super::read_box_header_ext(reader)?;

        let location = if flags == 0x1 {
            "local to file".to_string()
        } else {
            let buf_size = header
                .size
                .checked_sub(HEADER_SIZE + 4 + 1)
                .ok_or(Error::InvalidData("url size too small"))?;
            let mut buf = vec![0u8; buf_size as usize];
            reader.read_exact(&mut buf)?;

            match String::from_utf8(buf) {
                Ok(t) => {
                    if t.len() != buf_size as usize {
                        return Err(Error::InvalidData("string too small"));
                    }
                    t
                }
                _ => String::from(""),
            }
        };

        Ok(UrlBox {
            version,
            flags,
            location,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for UrlBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}
