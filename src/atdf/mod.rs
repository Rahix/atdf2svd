use std::collections::HashSet;

pub mod chip;
pub mod error;
pub mod field;
pub mod interrupt;
pub mod patch;
pub mod peripheral;
pub mod register;
pub mod register_group;
pub mod values;

pub fn parse<R: std::io::Read>(
    r: R,
    patches: &HashSet<String>,
) -> crate::Result<crate::chip::Chip> {
    let tree = xmltree::Element::parse(r)?;

    let mut chip = chip::parse(&tree)?;

    patch::signals_to_port_fields(&mut chip, &tree)
        .unwrap_or_else(|_| log::warn!("Could not apply 'signals_to_port_fields' patch!"));
    patch::remove_unsafe_cpu_regs(&mut chip, &tree)?;

    if patches.contains("remove_register_common_prefix") {
        patch::remove_register_common_prefix(&mut chip)?;
    }

    Ok(chip)
}
