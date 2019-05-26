pub mod chip;
pub mod error;
pub mod field;
pub mod patch;
pub mod peripheral;
pub mod register;
pub mod values;
pub mod interrupt;

pub fn parse<R: std::io::Read>(r: R) -> crate::Result<crate::chip::Chip> {
    let tree = xmltree::Element::parse(r)?;

    let mut chip = chip::parse(&tree)?;

    patch::signals_to_port_fields(&mut chip, &tree)?;
    patch::remove_unsafe_cpu_regs(&mut chip, &tree)?;

    Ok(chip)
}
