use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use super::{BoxHeader, BoxType, Ibox, ReadBox, WriteBox};
use crate::error::Error;
use crate::types::FixedPointU16;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Avc1Box {
    pub data_reference_index: u16,
    pub width: u16,
    pub height: u16,
    pub horizresolution: FixedPointU16,
    pub vertresolution: FixedPointU16,
    pub frame_count: u16,
    pub depth: u16,
}

impl Ibox for Avc1Box {
    fn typ(&self) -> BoxType {
        BoxType::Avc1
    }

    fn data_size(&self) -> u64 {
        8 + 70
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!(
            "data_reference_index={} width={} height={} frame_count={}",
            self.data_reference_index, self.width, self.height, self.frame_count
        );
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for Avc1Box {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        reader.read_u32::<BigEndian>()?; // reserved
        reader.read_u16::<BigEndian>()?; // reserved
        let data_reference_index = reader.read_u16::<BigEndian>()?;

        reader.read_u32::<BigEndian>()?; // pre-defined, reserved
        reader.read_u64::<BigEndian>()?; // pre-defined
        reader.read_u32::<BigEndian>()?; // pre-defined
        let width = reader.read_u16::<BigEndian>()?;
        let height = reader.read_u16::<BigEndian>()?;
        let horizresolution = FixedPointU16::new_raw(reader.read_u32::<BigEndian>()?);
        let vertresolution = FixedPointU16::new_raw(reader.read_u32::<BigEndian>()?);
        reader.read_u32::<BigEndian>()?; // reserved
        let frame_count = reader.read_u16::<BigEndian>()?;
        super::rel_skip(reader, 32)?; // compressorname
        let depth = reader.read_u16::<BigEndian>()?;
        reader.read_i16::<BigEndian>()?; // pre-defined

        Ok(Avc1Box {
            data_reference_index,
            width,
            height,
            horizresolution,
            vertresolution,
            frame_count,
            depth,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for Avc1Box {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AvcCBox {
    pub configuration_version: u8,
    pub avc_profile_indication: u8,
    pub profile_compatibility: u8,
    pub avc_level_indication: u8,
    pub length_size_minus_one: u8,
    pub sequence_parameter_sets: Vec<NalUnit>,
    pub picture_parameter_sets: Vec<NalUnit>,
}

impl Ibox for AvcCBox {
    fn typ(&self) -> BoxType {
        BoxType::AvcC
    }

    fn data_size(&self) -> u64 {
        self.picture_parameter_sets.iter().fold(
            self.sequence_parameter_sets
                .iter()
                .fold(7, |acc, e| acc + e.size()),
            |acc, e| acc + e.size(),
        )
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!(
            "avc_profile_indication={} avc_level_indication={}",
            self.avc_profile_indication, self.avc_level_indication
        );
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for AvcCBox {
    fn read(reader: &mut R, _: &BoxHeader) -> Result<Self> {
        let configuration_version = reader.read_u8()?;
        let avc_profile_indication = reader.read_u8()?;
        let profile_compatibility = reader.read_u8()?;
        let avc_level_indication = reader.read_u8()?;
        let length_size_minus_one = reader.read_u8()? & 0x3;
        let num_of_spss = reader.read_u8()? & 0x1F;
        let mut sequence_parameter_sets = Vec::with_capacity(num_of_spss as usize);
        for _ in 0..num_of_spss {
            let nal_unit = NalUnit::read(reader)?;
            sequence_parameter_sets.push(nal_unit);
        }
        let num_of_ppss = reader.read_u8()?;
        let mut picture_parameter_sets = Vec::with_capacity(num_of_ppss as usize);
        for _ in 0..num_of_ppss {
            let nal_unit = NalUnit::read(reader)?;
            picture_parameter_sets.push(nal_unit);
        }

        Ok(AvcCBox {
            configuration_version,
            avc_profile_indication,
            profile_compatibility,
            avc_level_indication,
            length_size_minus_one,
            sequence_parameter_sets,
            picture_parameter_sets,
        })
    }
}

impl<W: Write> WriteBox<&mut W> for AvcCBox {
    fn write(&self, _: &mut W, _: u64) -> Result<u64> {
        todo!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NalUnit {
    pub bytes: Vec<u8>,
}

impl NalUnit {
    fn size(&self) -> u64 {
        2 + self.bytes.len() as u64
    }

    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        let length = reader.read_u16::<BigEndian>()? as usize;
        let mut bytes = vec![0u8; length];
        reader.read_exact(&mut bytes)?;
        Ok(NalUnit { bytes })
    }
}
