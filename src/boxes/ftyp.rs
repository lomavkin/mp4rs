use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, FourCC, Ibox, ReadBox, WriteBox};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FtypBox {
    pub major_brand: FourCC,
    pub minor_version: u32,
    pub compatible_brands: Vec<FourCC>,
}

impl Ibox for FtypBox {
    fn typ(&self) -> BoxType {
        BoxType::Ftyp
    }

    fn data_size(&self) -> u64 {
        8 + (4 * self.compatible_brands.len() as u64)
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let mut compatible_brands = Vec::new();
        for brand in self.compatible_brands.iter() {
            compatible_brands.push(brand.to_string());
        }

        let compatible_brands = self
            .compatible_brands
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>();

        let s = format!(
            "major_brand={} minor_version={} compatible_brands={}",
            self.major_brand,
            self.minor_version,
            compatible_brands.join("-")
        );
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for FtypBox {
    fn read(reader: &mut R, header: &BoxHeader) -> Result<Self> {
        if header.size < 16 || header.size % 4 != 0 {
            return Err(Error::InvalidData("ftyp size too small or not aligned"));
        }

        let brand_count = (header.size - 16) / 4;
        let major = reader.read_u32::<BigEndian>()?;
        let minor = reader.read_u32::<BigEndian>()?;

        let mut brands = Vec::new();
        for _ in 0..brand_count {
            let b = reader.read_u32::<BigEndian>()?;
            brands.push(From::from(b));
        }

        Ok(FtypBox {
            major_brand: From::from(major),
            minor_version: minor,
            compatible_brands: brands,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for FtypBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!()
    }
}
