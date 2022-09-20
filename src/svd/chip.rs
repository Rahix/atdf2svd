use crate::chip;
use crate::svd;
use crate::ElementExt;

pub fn generate(c: &chip::Chip) -> crate::Result<xmltree::Element> {
    let mut el = xmltree::Element::new("device");

    el.attributes.insert("schemaVersion".to_string(), "1.1".to_string());
    el.attributes.insert("xmlns:xs".to_string(), "http://www.w3.org/2001/XMLSchema-instance".to_string());
    el.attributes.insert("xs:noNamespaceSchemaLocation".to_string(), "CMSIS-SVD.xsd".to_string());

    let defaults = [
        ("vendor", "Atmel"),
        ("name", c.name.as_ref()),
        ("description", c.description.as_deref().unwrap_or("No description available.")),
        ("addressUnitBits", "8"),
        ("size", "8"),
        ("access", "read-write"),
        ("resetValue", "0"),
        ("resetMask", "0xff"),
    ];
    for (name, value) in defaults.iter() {
        el.child_with_text(name, *value);
    }

    let cpu_tags = [
        ("name", "other"),
        ("revision", "r0p0"),
        ("endian", "little"),
        ("mpuPresent", "false"),
        ("fpuPresent", "false"),
        ("nvicPrioBits", "4"),
        ("vendorSystickConfig", "false"),
    ];
    let mut cpu = xmltree::Element::new("cpu");
    for (name, value) in cpu_tags.iter() {
        cpu.child_with_text(name, *value);
    }
    el.children.push(cpu);

    let mut peripherals = xmltree::Element::new("peripherals");
    peripherals.children = c
        .peripherals
        .values()
        .filter(has_registers)
        .map(svd::peripheral::generate)
        .collect::<Result<Vec<_>, _>>()?;
    if svd::interrupt::generate(&mut peripherals, c).is_err() {
        log::warn!("Could not generate CPU interrupts")
    }
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
