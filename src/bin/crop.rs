use image::io::Reader as ImageReader;

fn main() -> anyhow::Result<()> {
    let img = ImageReader::open("A.png")?.decode()?;
    img.crop_imm(0, 0, 150, 150).save("A_cropped.png")?;
    Ok(())
}