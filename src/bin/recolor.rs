use image::io::Reader as ImageReader;

fn main() -> anyhow::Result<()> {
    let mut img = ImageReader::open("A_cropped.png")?.decode()?.into_rgba8();
    for pixel in img.pixels_mut() {
        match pixel.0 {
            [0,0,0,0] => {}
            [0,0,0,255] => pixel.0 = [255,0,0,255],
            _ => pixel.0 = [0,0,0,0],
        }
    }
    img.save("A_cropped_recolored.png")?;
    Ok(())
}