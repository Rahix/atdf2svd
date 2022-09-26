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

    el.children.push(generate_cpu(c)?);

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

fn generate_cpu(c: &chip::Chip) -> crate::Result<xmltree::Element> {
    let mut cpu = xmltree::Element::new("cpu");

    let cpu_name = architecture_to_name(&c.architecture);
    let is_cortexm = cpu_name.starts_with("CM");

    let (nvic_prio_bits, vendor_systick_config) = match is_cortexm {
        true => ("4", "false"),
        // Non Cortex-M CPUs don't implement a NVIC nor a SysTick timer.
        false => ("0", "false"),
    };

    let defaults = [
        ("name", cpu_name.as_ref()),
        ("revision", "r0p0"),
        ("endian", "little"),
        ("mpuPresent", "false"),
        ("fpuPresent", "false"),
        ("nvicPrioBits", nvic_prio_bits),
        ("vendorSystickConfig", vendor_systick_config),
    ];
    for (name, value) in defaults.iter() {
        cpu.child_with_text(name, *value);
    }

    Ok(cpu)
}

fn architecture_to_name(architecture: &str) -> String {
    // Convert CORTEX-.* to C.* format. For example:
    //
    // - CORTEX-A5 -> CA5
    // - CORTEX-M0PLUS -> CM0PLUS
    let cortex_name = architecture.strip_prefix("CORTEX-").map(|suffix| {
        let mut name = String::with_capacity("C".len() + suffix.len());
        name.push_str("C");
        name.push_str(suffix);
        name
    });

    cortex_name.unwrap_or("other".to_string())
}
