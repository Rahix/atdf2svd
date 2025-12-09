pub mod chip;
pub mod field;
pub mod interrupt;
pub mod peripheral;
pub mod register;
pub mod restriction;

pub fn generate<W: std::io::Write>(c: &crate::chip::Chip, mut w: W) -> crate::Result<()> {
    let device = chip::generate(c)?;
    let svd_xml = svd_encoder::encode(&device)?;
    w.write_all(svd_xml.as_bytes())?;

    Ok(())
}
