use crate::chip;
use crate::ElementExt;

pub fn generate(f: &chip::Field) -> crate::Result<xmltree::Element> {
    let mut el = xmltree::Element::new("field");

    el.child_with_text("name", f.name.as_ref());
    el.child_with_text(
        "description",
        if let Some(ref desc) = f.description {
            desc.as_ref()
        } else {
            log::warn!("Description missing for field {:?}", f.name);
            "<TBD>"
        },
    );
    el.child_with_text("bitRange", format!("[{}:{}]", f.range.1, f.range.0));

    Ok(el)
}
