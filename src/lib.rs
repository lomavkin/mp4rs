use std::fs::File;
use std::io::BufReader;

mod error;
pub use error::Error;

mod types;
pub use types::*;

mod boxes;
pub use boxes::*;

mod reader;
pub use reader::*;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Scanning {
    Stop,
    Continue,
}

pub fn read_mp4_box(f: File) -> Result<Vec<Mp4BoxTree>> {
    let size = f.metadata()?.len();
    let mut reader = BufReader::new(f);
    let tree = reader::read_mp4_box(&mut reader, size)?;
    Ok(tree)
}

pub fn scan_mp4_box<C: FnMut(&BoxData) -> Scanning>(f: File, c: &mut C) -> Result<()> {
    let size = f.metadata()?.len();
    let mut reader = BufReader::new(f);
    Ok(reader::scan_mp4_box(&mut reader, size, c)?)
}

pub fn debug_dump_mp4_box(trees: &Vec<Mp4BoxTree>) {
    let mut stack = Vec::new();
    for c in trees.iter().rev() {
        stack.push((0, c));
    }

    while let Some(e) = stack.pop() {
        for _ in 0..e.0 {
            print!(" ")
        }
        print!("{:?}", e.1.node);
        for c in e.1.children.iter().rev() {
            stack.push((e.0 + 2, c));
        }
        println!();
    }
}
