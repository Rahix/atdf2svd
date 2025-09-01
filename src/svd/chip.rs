use crate::chip;

pub fn generate(c: &chip::Chip) -> crate::Result<svd_rs::Device> {
    let device = svd_rs::Device::builder()
        .xmlns_xs("http://www.w3.org/2001/XMLSchema-instance".to_string())
        .schema_version("1.1".to_string())
        .no_namespace_schema_location("CMSIS-SVD.xsd".to_string());

    let device = device
        .vendor(Some("Atmel".to_owned()))
        .name(c.name.clone())
        .version("1.0".to_string())
        .description(
            c.description
                .clone()
                .unwrap_or_else(|| "No description available.".to_string()),
        )
        .address_unit_bits(8)
        .width(8)
        .default_register_properties(
            svd_rs::RegisterProperties::new()
                .size(Some(8))
                .access(Some(svd_rs::Access::ReadWrite))
                .reset_value(Some(0))
                .reset_mask(Some(0xff)),
        );

    let device = device.cpu(Some(generate_cpu(c)?));

    let mut peripherals = c
        .peripherals
        .values()
        .filter(has_registers)
        .map(crate::svd::peripheral::generate)
        .collect::<Result<Vec<_>, _>>()?;

    if crate::svd::interrupt::generate(&mut peripherals, c).is_err() {
        log::warn!("Could not generate CPU interrupts");
    }

    device
        .peripherals(peripherals)
        .build(svd_rs::ValidateLevel::Strict)
        .map_err(crate::Error::from)
}

fn has_registers(peripheral: &&chip::Peripheral) -> bool {
    let regs = !peripheral.register_group.get_all_registers().is_empty();
    if !regs {
        log::warn!("No registers found for peripheral {}", peripheral.name);
    }
    regs
}

fn generate_cpu(c: &chip::Chip) -> crate::Result<svd_rs::Cpu> {
    let cpu_name = architecture_to_name(&c.architecture);

    svd_rs::Cpu::builder()
        .name(cpu_name)
        .revision("r0p0".to_string())
        .endian(svd_rs::Endian::Little)
        .mpu_present(false)
        .fpu_present(false)
        .nvic_priority_bits(4)
        .has_vendor_systick(false)
        .build(svd_rs::ValidateLevel::Strict)
        .map_err(crate::Error::from)
}

fn architecture_to_name(architecture: &str) -> String {
    // Convert CORTEX-.* to C.* format. For example:
    //
    // - CORTEX-A5 -> CA5
    // - CORTEX-M0PLUS -> CM0PLUS
    let cortex_name = architecture
        .strip_prefix("CORTEX-")
        .map(|suffix| format!("C{suffix}"));

    cortex_name.unwrap_or("other".to_string())
}
