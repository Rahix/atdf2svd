use crate::ElementExt;
use crate::atdf;
use crate::chip;
use std::collections::BTreeMap;

pub fn parse(el: &xmltree::Element) -> crate::Result<chip::Chip> {
    let devices = el.first_child("devices")?;
    if devices.children.len() != 1 {
        return Err(
            atdf::error::UnsupportedError::new("more than one device definition", devices).into(),
        );
    }

    let device = devices.first_child("device")?;

    let peripherals = atdf::peripheral::parse_list(
        device.first_child("peripherals")?,
        el.first_child("modules")?,
    )?
    .into_iter()
    .map(|p| (p.name.clone(), p))
    .collect();

    let interrupts_vec = device
        .first_child("interrupts")?
        .iter_children_with_name("interrupt", Some("interrupts"))
        .map(atdf::interrupt::parse)
        .collect::<Result<Vec<_>, _>>()?;

    // Check for duplicate index, merge names if duplicate index exists
    let mut interrupts = BTreeMap::<usize, chip::Interrupt>::new();
    for int in interrupts_vec {
        if let Some(existing_int) = interrupts.get_mut(&int.index) {
            let old_name = existing_int.name.clone();
            if let Some(split_idx) = int.name.find('_') {
                existing_int.name.push_str(int.name.split_at(split_idx).1);
            } else {
                existing_int.name.push('_');
                existing_int.name.push_str(&int.name);
            }
            log::warn!(
                "Merging interrupt {} and {} to {}",
                old_name,
                int.name,
                existing_int.name
            );
        } else {
            interrupts.insert(int.index, int);
        }
    }

    // Map interrupts from <usize, chip::Interrupt> to <std::string::String, chip::Interrupt>
    let interrupts = interrupts
        .values()
        .map(|int| (int.name.clone(), int.clone()))
        .collect();

    Ok(chip::Chip {
        name: device.attr("name")?.clone(),
        architecture: device.attr("architecture")?.clone(),
        family: device.attr("family")?.clone(),
        series: device.attr("series").ok().cloned(),

        description: None,
        vendor: None,
        version: None,

        peripherals,
        interrupts,
    })
}
