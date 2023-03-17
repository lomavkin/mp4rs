use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::convert::TryInto;
use std::fmt;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::error::Error;
use crate::types::FourCC;

pub(crate) mod avc1;
pub(crate) mod co64;
pub(crate) mod ctts;
pub(crate) mod data;
pub(crate) mod dinf;
pub(crate) mod dref;
pub(crate) mod edts;
pub(crate) mod elst;
pub(crate) mod emsg;
pub(crate) mod ftyp;
pub(crate) mod hdlr;
pub(crate) mod hev1;
pub(crate) mod ilst;
pub(crate) mod mdhd;
pub(crate) mod mdia;
pub(crate) mod mehd;
pub(crate) mod meta;
pub(crate) mod mfhd;
pub(crate) mod minf;
pub(crate) mod moof;
pub(crate) mod moov;
pub(crate) mod mp4a;
pub(crate) mod mvex;
pub(crate) mod mvhd;
pub(crate) mod pasp;
pub(crate) mod phtm;
pub(crate) mod smhd;
pub(crate) mod stbl;
pub(crate) mod stco;
pub(crate) mod stsc;
pub(crate) mod stsd;
pub(crate) mod stss;
pub(crate) mod stsz;
pub(crate) mod stts;
pub(crate) mod tfdt;
pub(crate) mod tfhd;
pub(crate) mod tkhd;
pub(crate) mod traf;
pub(crate) mod trak;
pub(crate) mod trex;
pub(crate) mod trun;
pub(crate) mod tx3g;
pub(crate) mod udta;
pub(crate) mod url;
pub(crate) mod vmhd;
pub(crate) mod vp09;
pub(crate) mod vpcc;

type Result<T> = std::result::Result<T, Error>;

pub const HEADER_SIZE: u64 = 8;
pub const HEADER_SIZE_LARGE: u64 = 16;

#[derive(Debug, Clone)]
pub struct Mp4BoxTree {
    pub node: Mp4Box,
    pub children: Vec<Mp4BoxTree>,
}

#[derive(Debug, Clone)]
pub struct Mp4Box {
    pub header: BoxHeader,
    pub data: BoxData,
}

macro_rules! boxdef {
    ( $( $mod:ident, $field:ident, $box:ident => $value:expr, )* ) => {
        boxdef!{ $( $mod, $field, $box => $value ),* }
    };
    ( $( $mod:ident, $field:ident, $box:ident => $value:expr ),* ) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum BoxType {
            $( $field, )*
            UnKnown(u32),
        }

        #[derive(Debug, Clone)]
        pub enum BoxData {
            $( $field($box) ),*
        }

        impl BoxData {
            pub fn typ(&self) -> BoxType {
                match &self {
                    $( BoxData::$field(ref b) => b.typ() ),*
                }
            }

            pub fn effective_size(&self) -> u64 {
                match &self {
                    $( BoxData::$field(ref b) => b.header_size() + b.data_size() ),*
                }
            }
        }

        $(
            impl<'a> TryFrom<&'a BoxData> for &'a $box {
                type Error = Error;

                fn try_from(d: &'a BoxData) -> Result<Self> {
                    match *d {
                        BoxData::$field(ref b) => Ok(b),
                        _ => Err(Error::BoxNotFound(BoxType::UnKnown(0))),
                    }
                }
            }

            impl <'a> TryFrom<&'a Mp4Box> for &'a $box {
                type Error = Error;

                fn try_from(b: &'a Mp4Box) -> Result<Self> {
                    TryFrom::try_from(&b.data)
                }
            }

            impl <'a> TryFrom<&'a Mp4BoxTree> for &'a $box {
                type Error = Error;

                fn try_from(tree: &'a Mp4BoxTree) -> Result<Self> {
                    TryFrom::try_from(&tree.node.data)
                }
            }
        )*

        impl From<u32> for BoxType {
            fn from(t: u32) -> Self {
                match t {
                    $( $value => BoxType::$field, )*
                    _ => BoxType::UnKnown(t),
                }
            }
        }

        impl From<BoxType> for u32 {
            fn from(b: BoxType) -> Self {
                match b {
                    $( BoxType::$field => $value, )*
                    BoxType::UnKnown(t) => t,
                }
            }
        }

        $( pub use $mod::$box; )*
    };
}

boxdef! {
    phtm, Phtm, PhtmBox => 0x2d_2d_2d_2d,
    ftyp, Ftyp, FtypBox => 0x66_74_79_70,
    mvhd, Mvhd, MvhdBox => 0x6d_76_68_64,
    // mfhd, Mfhd, MfhdBox => 0x6d_66_68_64,
    // free, Free, FreeBox => 0x66_72_65_65,
    // mdat, Mdat, MdatBox => 0x6d_64_61_74,
    moov, Moov, MoovBox => 0x6d_6f_6f_76,
    // mvex, Mvex, MvexBox => 0x6d_76_65_78,
    // mehd, Mehd, MehdBox => 0x6d_65_68_64,
    // trex, Trex, TrexBox => 0x74_72_65_78,
    // emsg, Emsg, EmsgBox => 0x65_6d_73_67,
    // moof, Moof, MoofBox => 0x6d_6f_6f_66,
    tkhd, Tkhd, TkhdBox => 0x74_6b_68_64,
    // tfhd, Tfhd, TfhdBox => 0x74_66_68_64,
    // tfdt, Tfdt, TfdtBox => 0x74_66_64_74,
    edts, Edts, EdtsBox => 0x65_64_74_73,
    mdia, Mdia, MdiaBox => 0x6d_64_69_61,
    // elst, Elst, ElstBox => 0x65_6c_73_74,
    mdhd, Mdhd, MdhdBox => 0x6d_64_68_64,
    hdlr, Hdlr, HdlrBox => 0x68_64_6c_72,
    minf, Minf, MinfBox => 0x6d_69_6e_66,
    vmhd, Vmhd, VmhdBox => 0x76_6d_68_64,
    stbl, Stbl, StblBox => 0x73_74_62_6c,
    stsd, Stsd, StsdBox => 0x73_74_73_64,
    stts, Stts, SttsBox => 0x73_74_74_73,
    ctts, Ctts, CttsBox => 0x63_74_74_73,
    stss, Stss, StssBox => 0x73_74_73_73,
    stsc, Stsc, StscBox => 0x73_74_73_63,
    stsz, Stsz, StszBox => 0x73_74_73_7A,
    stco, Stco, StcoBox => 0x73_74_63_6F,
    // co64, Co64, Co64Box => 0x63_6F_36_34,
    trak, Trak, TrakBox => 0x74_72_61_6b,
    // traf, Traf, TrafBox => 0x74_72_61_66,
    // trun, Trun, TrunBox => 0x74_72_75_6E,
    // udta, Udta, UdtaBox => 0x75_64_74_61,
    // meta, Meta, MetaBox => 0x6d_65_74_61,
    dinf, Dinf, DinfBox => 0x64_69_6e_66,
    dref, Dref, DrefBox => 0x64_72_65_66,
    url, Url, UrlBox => 0x75_72_6c_20,
    // dref, Dref, DrefBox => 0x64_72_65_66,
    // url, Url, UrlBox  => 0x75_72_6C_20,
    smhd, Smhd, SmhdBox => 0x73_6d_68_64,
    avc1, Avc1, Avc1Box => 0x61_76_63_31,
    avc1, AvcC, AvcCBox => 0x61_76_63_43,
    pasp, Pasp, PaspBox => 0x70_61_73_70,
    // hev1, Hev1, Hev1Box => 0x68_65_76_31,
    // hvcc, HvcC, HvcCBox => 0x68_76_63_43,
    // mp4a, Mp4a, Mp4aBox => 0x6d_70_34_61,
    // esds, Esds, EsdsBox => 0x65_73_64_73,
    // tx3g, Tx3g, Tx3gBox => 0x74_78_33_67,
    // vpcc, Vpcc, VpccBox => 0x76_70_63_43,
    // vp09, Vp09, Vp09Box => 0x76_70_30_39,
    // data, Data, DataBox => 0x64_61_74_61,
    // ilst, Ilst, IlstBox => 0x69_6c_73_74,
    // name, Name, NameBox => 0xa9_6e_61_6d,
    // day, Day, DayBox => 0xa9_64_61_79,
    // covr, Covr, CovrBox => 0x63_6f_76_72,
    // desc, Desc, DescBox => 0x64_65_73_63,
    // wide, Wide, WideBox => 0x77_69_64_65,
}

impl fmt::Debug for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fourcc: FourCC = From::from(*self);
        write!(f, "{fourcc}")
    }
}

pub trait Ibox {
    fn typ(&self) -> BoxType;

    fn header_size(&self) -> u64 {
        HEADER_SIZE
    }

    fn data_size(&self) -> u64;

    fn to_json(&self) -> Result<String>;

    fn summary(&self) -> Result<String>;
}

pub trait ReadBox<T>: Sized {
    fn read(_: T, header: &BoxHeader) -> Result<Self>;
}

pub trait WriteBox<T> {
    fn write(&self, _: T, offset: u64) -> Result<u64>;
}

#[derive(Debug, Clone, Copy)]
pub struct BoxHeader {
    pub typ: BoxType,
    pub size: u64,
    pub offset: u64,
}

impl BoxHeader {
    pub fn new(typ: BoxType, size: u64, offset: u64) -> BoxHeader {
        BoxHeader { typ, size, offset }
    }

    pub fn box_start(&self) -> u64 {
        self.offset
    }

    pub fn skip_box<S: Seek>(&self, seeker: &mut S) -> Result<()> {
        let start = self.box_start();
        abs_skip(seeker, start + self.size)?;
        Ok(())
    }
}

pub fn read_box_header<R: Read>(reader: &mut R, offset: u64) -> Result<BoxHeader> {
    let mut buf = [0_u8; HEADER_SIZE as usize];
    reader.read_exact(&mut buf)?;

    let s = buf[0..4].try_into().unwrap();
    let size = u32::from_be_bytes(s);

    let t = buf[4..8].try_into().unwrap();
    let typ = u32::from_be_bytes(t);

    if size == 1 {
        reader.read_exact(&mut buf)?;
        let largesize = u64::from_be_bytes(buf);

        Ok(BoxHeader {
            typ: BoxType::from(typ),
            size: match largesize {
                0 => 0,
                1..=15 => return Err(Error::InvalidData("64 bit size too small")),
                16..=u64::MAX => largesize,
            },
            offset,
        })
    } else {
        Ok(BoxHeader {
            typ: BoxType::from(typ),
            size: size as u64,
            offset,
        })
    }
}

pub fn read_box_header_ext<R: Read>(reader: &mut R) -> Result<(u8, u32)> {
    let version = reader.read_u8()?;
    let flags = reader.read_u24::<BigEndian>()?;
    Ok((version, flags))
}

#[allow(dead_code)]
pub fn write_box_header<W: Write>(header: &BoxHeader, writer: &mut W) -> Result<u64> {
    if header.size > u32::MAX as u64 {
        writer.write_u32::<BigEndian>(1)?;
        writer.write_u32::<BigEndian>(header.typ.into())?;
        writer.write_u64::<BigEndian>(header.size)?;
        Ok(HEADER_SIZE_LARGE)
    } else {
        writer.write_u32::<BigEndian>(header.size as u32)?;
        writer.write_u32::<BigEndian>(header.typ.into())?;
        Ok(HEADER_SIZE)
    }
}

#[allow(dead_code)]
pub fn write_box_header_ext<W: Write>(writer: &mut W, version: u8, flags: u32) -> Result<u64> {
    writer.write_u8(version)?;
    writer.write_u24::<BigEndian>(flags)?;
    Ok(4)
}

pub fn rel_skip<S: Seek>(seeker: &mut S, size: u64) -> Result<()> {
    seeker.seek(SeekFrom::Current(size as i64))?;
    Ok(())
}

pub fn abs_skip<S: Seek>(seeker: &mut S, size: u64) -> Result<()> {
    seeker.seek(SeekFrom::Start(size))?;
    Ok(())
}
