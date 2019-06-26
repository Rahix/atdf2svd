use crate::chip;
use crate::svd;
use crate::ElementExt;

pub fn generate(p: &chip::Peripheral) -> crate::Result<xmltree::Element> {
    let mut el = xmltree::Element::new("peripheral");
    let base = p.base_address().expect("todo error");

    el.child_with_text("name", p.name.clone());
    el.child_with_text(
        "description",
        if let Some(ref desc) = p.description {
            desc.as_ref()
        } else {
            log::warn!("Description missing for peripheral {:?}", p.name);
            "<TBD>"
        },
    );
    el.child_with_text("baseAddress", format!("0x{:X}", base));

    let mut registers = xmltree::Element::new("registers");

    registers.children = p
        .registers
        .values()
        .map(|r| svd::register::generate(r, base))
        .collect::<Result<Vec<_>, _>>()?;

    el.children.push(registers);

    Ok(el)
}
