//! Patches for atdf files that can generally be applied
use crate::chip;
use crate::util;
use crate::ElementExt;
use std::collections::BTreeMap;

const NEW_PORT_REGS: [&str; 10] = [
    "DIR", "DIRSET", "DIRCLR", "DIRTGL", "OUT", "OUTSET", "OUTCLR", "OUTTGL", "IN", "INTFLAGS",
];

pub fn signals_to_port_fields(chip: &mut chip::Chip, tree: &xmltree::Element) -> crate::Result<()> {
    let port_module = tree
        .first_child("devices")?
        .first_child("device")?
        .first_child("peripherals")?
        .first_child_by_attr(Some("module"), "name", "PORT")?;

    for port in chip
        .peripherals
        .values_mut()
        .filter(|p| p.name.starts_with("PORT") && p.name.len() == 5)
    {
        let name = port.name.chars().rev().next().unwrap();

        let pins: Vec<_> = port_module
            .first_child_by_attr(Some("instance"), "name", &port.name)?
            .first_child("signals")?
            .children
            .iter()
            .filter_map(|node| node.as_element())
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
            if reg.name.ends_with(name) || NEW_PORT_REGS.iter().any(|r| r == &reg.name) {
                reg.fields = fields.clone();
                // Ensure that direct access to the register is unsafe
                reg.restriction = chip::ValueRestriction::Unsafe;
            }
        }
    }
    Ok(())
}

pub fn port_rename_snake_case(chip: &mut chip::Chip) -> crate::Result<()> {
    let peripherals = std::mem::take(&mut chip.peripherals);
    chip.peripherals = peripherals
        .into_iter()
        .map(|(name, mut port)| {
            if port.name.starts_with("PORT") && port.name.len() == 5 {
                let new_name = format!("PORT_{}", name.chars().last().unwrap());
                log::debug!("[port_rename_snake_case] Renaming {name} to {new_name} ...");

                port.name = new_name.clone();

                (new_name, port)
            } else {
                (name, port)
            }
        })
        .collect();

    Ok(())
}

pub fn remove_unsafe_cpu_regs(chip: &mut chip::Chip, _el: &xmltree::Element) -> crate::Result<()> {
    if let Some(cpu) = chip.peripherals.get_mut("CPU") {
        cpu.registers.remove("SREG");
        cpu.registers.remove("SP");
    }

    Ok(())
}

fn longest_common_prefix<'a>(strings: &[&'a str]) -> &'a str {
    if strings.is_empty() {
        return "";
    }

    let mut longest_prefix = "";
    for prefix in strings[0]
        .char_indices()
        .map(|(i, _)| strings[0].split_at(i).0)
    {
        if strings.iter().all(|s| s.starts_with(prefix)) {
            longest_prefix = prefix;
        } else {
            // This prefix no longer matches so the previous one was the longest.
            break;
        }
    }
    longest_prefix
}

pub fn remove_register_common_prefix(chip: &mut chip::Chip) -> crate::Result<()> {
    for peripheral in chip.peripherals.values_mut() {
        // There's not enough quorum in less than two elements to find a
        // prefix.
        if peripheral.registers.len() < 2 {
            continue;
        }

        let register_names: Vec<_> = peripheral.registers.keys().map(String::as_str).collect();
        let common_prefix = longest_common_prefix(&register_names).to_string();

        let is_valid_prefix = common_prefix.ends_with("_") && common_prefix.chars().count() >= 2;
        if is_valid_prefix {
            for register in peripheral.registers.values_mut() {
                if let Some(s) = register.name.strip_prefix(&common_prefix) {
                    log::debug!(
                        "[remove_register_common_prefix] Renaming {} to {}",
                        register.name,
                        s
                    );
                    register.name = s.to_string();
                }
            }
        }
    }

    Ok(())
}
