use mp4box::{self, MoovBox};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("sample-5s.mp4")?;
    if let Ok(trees) = mp4box::read_mp4_box(f) {
        for tree in trees.iter() {
            if let Ok(moov) = TryInto::<&MoovBox>::try_into(tree) {
                println!("{:?}", moov);
            }
        }
        mp4box::debug_dump_mp4_box(&trees);
    }
    Ok(())
}
