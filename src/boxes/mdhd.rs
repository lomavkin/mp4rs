use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::char::{self, REPLACEMENT_CHARACTER};
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox, HEADER_SIZE};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MdhdBox {
    pub version: u8,
    pub flags: u32,
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub language: String,
}

impl Ibox for MdhdBox {
    fn typ(&self) -> BoxType {
        BoxType::Mdhd
    }

    fn header_size(&self) -> u64 {
        HEADER_SIZE + 4
    }

    fn data_size(&self) -> u64 {
        let mut size = if self.version == 1 { 28 } else { 16 };
        size += 4;
        size
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!(
            "creation_time={} timescale={} duration={} language={}",
            self.creation_time, self.timescale, self.duration, self.language
        );
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for MdhdBox {
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
        let language_code = reader.read_u16::<BigEndian>()?;
        let language = language_string(language_code);

        Ok(MdhdBox {
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for MdhdBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}

fn language_string(language: u16) -> String {
    let mut lang: [u16; 3] = [0; 3];

    lang[0] = ((language >> 10) & 0x1F) + 0x60;
    lang[1] = ((language >> 5) & 0x1F) + 0x60;
    lang[2] = ((language) & 0x1F) + 0x60;

    // Decode utf-16 encoded bytes into a string.
    let lang_str = char::decode_utf16(lang.iter().cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect::<String>();

    lang_str
}
