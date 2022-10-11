use crate::chip;
use crate::DisplayError;
use std::convert::TryInto;

pub fn generate(peripherals: &mut [svd_rs::Peripheral], c: &chip::Chip) -> crate::Result<()> {
    // Find the (first) peripheral with the name `CPU` to then add the interrupts to it.
    for peripheral in peripherals.iter_mut() {
        if peripheral.name == "CPU" {
            let mut interrupts = c.interrupts.values().collect::<Vec<_>>();
            interrupts.sort_by(|a, b| a.index.cmp(&b.index));

            let interrupts = interrupts
                .into_iter()
                .map(|interrupt| {
                    svd_rs::Interrupt::builder()
                        .name(interrupt.name.clone())
                        .description(interrupt.description.clone())
                        .value(interrupt.index.try_into().map_err(crate::Error::from)?)
                        .build(svd_rs::ValidateLevel::Strict)
                        .map_err(crate::Error::from)
                })
                .collect::<Result<Vec<_>, _>>()?;

            peripheral.interrupt.extend(interrupts);
            return Ok(());
        }
    }
    // No peripheral named `CPU` found.
    Err(NoCPUPeripheral.into())
}

struct NoCPUPeripheral;

impl DisplayError for NoCPUPeripheral {
    fn format(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(
            w,
            "No `CPU` peripheral found, to which interrupts can be added!"
        )
    }
}
