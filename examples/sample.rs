use mp4box::{AvcCBox, Scanning};
use std::fs::File;

fn main() -> std::io::Result<()> {
    {
        // read_mp4_box
        let f = File::open("sample-5s.mp4")?;
        if let Ok(trees) = mp4box::read_mp4_box(f) {
            mp4box::debug_dump_mp4_box(&trees);
        }
    }
    {
        // scan_mp4_box
        let f = File::open("sample-5s.mp4")?;
        let _ = mp4box::scan_mp4_box(f, &mut |b| {
            if let Ok(avc_c) = TryInto::<&AvcCBox>::try_into(b) {
                println!("{:?}", avc_c);
                Scanning::Stop
            } else {
                Scanning::Continue
            }
        });
    }
    Ok(())
}
