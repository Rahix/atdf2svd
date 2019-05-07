//! Patches for atdf files that can generally be applied
use crate::chip;
use crate::util;
use crate::ElementExt;
use std::collections::HashMap;

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

        let fields: HashMap<String, chip::Field> = pins
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
            if !reg.name.ends_with(name) {
                log::error!("Register {:?} has a weird name!", reg.name);
            }

            reg.fields = fields.clone();
        }
    }
    Ok(())
}

pub fn remove_unsafe_cpu_regs(chip: &mut chip::Chip, _el: &xmltree::Element) -> crate::Result<()> {
    if let Some(cpu) = chip.peripherals.get_mut("CPU") {
        cpu.registers.remove("SREG");
        cpu.registers.remove("SP");
    }

    Ok(())
}
