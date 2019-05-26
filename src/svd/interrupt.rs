use crate::chip;
use crate::elementext::ElementExt;
use crate::DisplayError;

pub fn generate(peripherals: &mut xmltree::Element, c: &chip::Chip) -> crate::Result<()> {
    // Find the (first) peripheral with the name `CPU` to then add the interrupts to it.
    for peripheral in peripherals.children.iter_mut() {
        if peripheral.get_child("name".to_string()).and_then(|e| {
            // Once `inner_deref` is stabilized, this can be changed to
            // e.text.deref() or e.text.as_deref(), depending on the final name
            e.text.as_ref().map(|s| s.as_ref())
        }) == Some("CPU")
        {
            let mut interrupts = c.interrupts.values().collect::<Vec<_>>();
            interrupts.sort_by(|a, b| a.index.cmp(&b.index));
            let el_interrupts = interrupts.into_iter()
                .map(|interrupt| {
                    let mut int = xmltree::Element::new("interrupt");

                    int.child_with_text(
                        "name",
                        interrupt.name.as_ref(),
                    );
                    int.child_with_text(
                        "value",
                        interrupt.index.to_string(),
                    );

                    int.child_with_text(
                        "description",
                        if let Some(ref desc) = interrupt.description {
                            desc.as_ref()
                        } else {
                            log::warn!("Description missing for field {:?}", interrupt.name);
                            "<TBD>"
                        },
                    );

                    int
                });

            peripheral
                .children
                .extend(el_interrupts);

            return Ok(());
        }
    }
    // No peripheral named `CPU` found.
    Err(NoCPUPeripheral.into())
}

struct NoCPUPeripheral;

impl DisplayError for NoCPUPeripheral {
    fn format(&self, w: &mut std::io::Write) -> std::io::Result<()> {
        write!(
            w,
            "No `CPU` peripheral found, to which interrupts can be added!"
        )
    }
}
