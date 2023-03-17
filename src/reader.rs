use std::io::{Read, Seek};

use crate::boxes::{self, BoxData, BoxHeader, BoxType, Mp4Box, Mp4BoxTree, PhtmBox, ReadBox};
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

macro_rules! dispatch {
    ( $self:expr, $reader:expr, $header:expr; $( $field:ident => $value:ident, )* ) => {
        dispatch!( $self, $reader, $header; $( $field => $value ),* )
    };
    ( $self:expr, $reader:expr, $header:expr; $( $field:ident => $value:ident ),* ) => {
        match $header.typ {
            $(
                BoxType::$field => {
                    let b = boxes::$value::read($reader, &$header)?;
                    let data = BoxData::$field(b);
                    $self.traverse($reader, $header, data)?;
                }
            )*
            _ => $header.skip_box($reader)?,
        }
    };
}

impl Mp4BoxTree {
    fn new(mp4box: Mp4Box) -> Self {
        Self {
            node: mp4box,
            children: Vec::new(),
        }
    }

    fn traverse<R: Read + Seek>(
        &mut self,
        reader: &mut R,
        header: BoxHeader,
        data: BoxData,
    ) -> Result<()> {
        let skip_size = data.effective_size();
        let box_start = header.box_start() + skip_size;
        boxes::abs_skip(reader, box_start)?;

        let mp4 = Mp4Box::new(header, data);
        let mut tree = Self::new(mp4);
        let remain = header.size - skip_size;
        if remain > boxes::HEADER_SIZE {
            let new_tree = tree.read(reader, box_start + remain)?;
            self.children.push(new_tree);
        } else {
            if remain > 0 {
                // skip because readable data is left but it is less than header size
                boxes::rel_skip(reader, remain)?;
            }
            self.children.push(tree);
        }
        Ok(())
    }

    fn read<R: Read + Seek>(&mut self, reader: &mut R, size: u64) -> Result<Self> {
        let start = reader.stream_position()?;

        let mut current = start;
        while current < size {
            let header = boxes::read_box_header(reader, current)?;
            if header.size > size {
                return Err(Error::InvalidData(
                    "file contains a box with a larger size than it",
                ));
            }

            if header.size == 0 {
                break;
            }

            dispatch! {
                self, reader, header;
                Ftyp => FtypBox,
                Moov => MoovBox,
                Mvhd => MvhdBox,
                Trak => TrakBox,
                Tkhd => TkhdBox,
                Edts => EdtsBox,
                Mdia => MdiaBox,
                Mdhd => MdhdBox,
                Hdlr => HdlrBox,
                Minf => MinfBox,
                Smhd => SmhdBox,
                Vmhd => VmhdBox,
                Dinf => DinfBox,
                Dref => DrefBox,
                Url  => UrlBox,
                Stbl => StblBox,
                Stsd => StsdBox,
                Avc1 => Avc1Box,
                AvcC => AvcCBox,
                Pasp => PaspBox,
                Stts => SttsBox,
                Ctts => CttsBox,
                Stss => StssBox,
                Stsc => StscBox,
                Stsz => StszBox,
                Stco => StcoBox,
            };

            current = reader.stream_position()?;
        }
        Ok(self.clone())
    }

    pub fn node_header_ref(&self) -> &BoxHeader {
        &self.node.header
    }

    pub fn node_data_ref(&self) -> &BoxData {
        &self.node.data
    }
}

impl Mp4Box {
    fn new(header: BoxHeader, data: BoxData) -> Mp4Box {
        Mp4Box { header, data }
    }

    fn phtm() -> Mp4Box {
        Mp4Box {
            header: BoxHeader {
                typ: BoxType::Phtm,
                size: 0,
                offset: 0,
            },
            data: BoxData::Phtm(PhtmBox),
        }
    }
}

pub fn read_mp4_box<R: Read + Seek>(reader: &mut R, size: u64) -> Result<Vec<Mp4BoxTree>> {
    let phtm = Mp4Box::phtm();
    let mut tree = Mp4BoxTree::new(phtm);
    let root = tree.read(reader, size)?;
    Ok(root.children)
}
