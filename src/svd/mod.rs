pub mod chip;
pub mod field;
pub mod peripheral;
pub mod register;
pub mod restriction;
pub mod interrupt;

pub fn generate<W: std::io::Write>(c: &crate::chip::Chip, mut w: W) -> crate::Result<()> {
    let tree = chip::generate(c)?;

    let config = xmltree::EmitterConfig::new().perform_indent(true);

    tree.write_with_config(&mut w, config)?;
    write!(&mut w, "\n")?;

    Ok(())
}
