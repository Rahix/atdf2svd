//! Patches for atdf files that can generally be applied
use crate::chip;
use crate::util;
use crate::ElementExt;
use std::collections::BTreeMap;

pub fn signals_to_port_fields(chip: &mut chip::Chip, tree: &xmltree::Element) -> crate::Result<()> {
    let port_module = tree
        .first_child("devices")?
        .first_child("device")?
        .first_child("peripherals")?
        .first_child_by_attr(Some("module"), "name", "PORT")?;

    for port in chip
        .peripherals
        .values_mut()
        .filter(|p| p.name.starts_with("PORT"))
    {
        let name = port.name.chars().rev().next().unwrap();

        let pins: Vec<_> = port_module
            .first_child_by_attr(Some("instance"), "name", &port.name)?
            .first_child("signals")?
            .children
            .iter()
            .map(|el| el.attr("index"))
            .map(|r| r.and_then(|s| util::parse_int(s)))
            .collect::<Result<_, _>>()?;

        let fields: BTreeMap<String, chip::Field> = pins
            .into_iter()
            .map(|p| chip::Field {
                name: format!("P{}{}", name, p),
                description: Some(format!("Pin {}{}", name, p)),
                range: (p, p),
                access: chip::AccessMode::ReadWrite,
                restriction: chip::ValueRestriction::Any,
            })
            .map(|f| (f.name.clone(), f))
            .collect();

        for reg in port.registers.values_mut() {
            if !correct_reg_name(&reg.name, name) {
                log::error!("Register {:?} has a weird name!", reg.name);
            }

            reg.fields = fields.clone();
            // Ensure that direct access to the register is unsafe
            reg.restriction = chip::ValueRestriction::Unsafe;
        }
    }
    Ok(())
}

fn correct_reg_name(reg_name: &str, port_name: char) -> bool {
    reg_name.ends_with(port_name)
        || reg_name.ends_with("CTRL")
        || reg_name.starts_with("DIR")
        || reg_name.starts_with("OUT")
        || reg_name.starts_with("IN")
}

pub fn remove_unsafe_cpu_regs(chip: &mut chip::Chip, _el: &xmltree::Element) -> crate::Result<()> {
    if let Some(cpu) = chip.peripherals.get_mut("CPU") {
        cpu.registers.remove("SREG");
        cpu.registers.remove("SP");
    }

    Ok(())
}
