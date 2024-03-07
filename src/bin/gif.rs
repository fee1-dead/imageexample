use std::fs::OpenOptions;
use std::io::BufWriter;

use image::codecs::gif::{GifEncoder, Repeat};
use image::io::Reader as ImageReader;
use image::{DynamicImage, Frame};

fn main() -> anyhow::Result<()> {
    let img = ImageReader::open("A_cropped_recolored_huerotated.png")?.decode()?;
    let frames = (0..360).map(|angle| img.huerotate(angle)).map(DynamicImage::into_rgba8).map(Frame::new);
    let file = OpenOptions::new().create(true).write(true).open("A_cropped_recolored_huerotated.gif")?;
    let buf = BufWriter::new(file);
    let mut encoder = GifEncoder::new_with_speed(buf, 20);
    encoder.set_repeat(Repeat::Infinite)?;
    encoder.encode_frames(frames)?;
    img.save("A_cropped_recolored.png")?;
    Ok(())
}