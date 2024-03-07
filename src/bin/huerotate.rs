use image::io::Reader as ImageReader;

fn main() -> anyhow::Result<()> {
    let img = ImageReader::open("A_cropped_recolored.png")?.decode()?;
    img.huerotate(120).save("A_cropped_recolored_huerotated.png")?;
    Ok(())
}