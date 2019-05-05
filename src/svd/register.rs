use crate::chip;
use crate::ElementExt;

pub fn generate(r: &chip::Register, base: usize) -> crate::Result<xmltree::Element> {
    let mut el = xmltree::Element::new("register");

    el.child_with_text("name", r.name.as_ref());
    el.child_with_text(
        "description",
        if let Some(ref desc) = r.description {
            desc.as_ref()
        } else {
            log::warn!("Description missing for register {:?}", r.name);
            "<TBD>"
        },
    );
    el.child_with_text("addressOffset", format!("0x{:X}", r.address - base));

    if let Some(mode) = match r.access {
        chip::AccessMode::ReadOnly => Some("read-only"),
        chip::AccessMode::WriteOnly => Some("write-only"),
        _ => None,
    } {
        el.child_with_text("access", mode);
    }

    Ok(el)
}
