use crate::chip;
use crate::svd;
use crate::ElementExt;

pub fn generate(c: &chip::Chip) -> crate::Result<xmltree::Element> {
    let mut el = xmltree::Element::new("device");

    let defaults = [
        ("vendor", "Atmel"),
        ("name", c.name.as_ref()),
        ("addressUnitBits", "8"),
        ("size", "8"),
        ("access", "read-write"),
        ("resetValue", "0"),
        ("resetMask", "0xff"),
    ];

    for (name, value) in defaults.iter() {
        el.child_with_text(name, *value);
    }

    let mut peripherals = xmltree::Element::new("peripherals");

    peripherals.children = c
        .peripherals
        .values()
        .filter(has_registers)
        .map(svd::peripheral::generate)
        .collect::<Result<Vec<_>, _>>()?;

    svd::interrupt::generate(&mut peripherals, c)?;

    el.children.push(peripherals);

    Ok(el)
}

fn has_registers(peripheral: &&chip::Peripheral) -> bool {
    let regs = !peripheral.registers.is_empty();
    if !regs {
        log::warn!("No registers found for peripheral {:?}", peripheral.name);
    }
    regs
}
