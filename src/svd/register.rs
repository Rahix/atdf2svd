use crate::chip;
use crate::ElementExt;

pub fn generate(r: &chip::Register, base: usize) -> crate::Result<xmltree::Element> {
    let mut el = xmltree::Element::new("register");

    el.children
        .push(xmltree::Element::new_with_text("name", r.name.as_ref()));
    el.children.push(xmltree::Element::new_with_text(
        "description",
        if let Some(ref desc) = r.description {
            desc.as_ref()
        } else {
            "<TBD>"
        },
    ));
    el.children.push(xmltree::Element::new_with_text(
        "addressOffset",
        format!("0x{:X}", r.address - base),
    ));

    if let Some(mode) = match r.access {
        chip::AccessMode::ReadOnly => Some("read-only"),
        chip::AccessMode::WriteOnly => Some("write-only"),
        _ => None,
    } {
        el.children
            .push(xmltree::Element::new_with_text("access", mode));
    }

    Ok(el)
}
