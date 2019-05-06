use crate::chip;
use crate::svd;
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

    if r.size != 1 {
        el.child_with_text("size", (r.size * 8).to_string());
    }

    if let Some(a) = svd::restriction::generate_access(&r.access)? {
        el.children.push(a);
    }

    el.children
        .extend(svd::restriction::generate(&r.restriction, r.size * 8)?);

    if r.fields.len() > 0 {
        let mut fields_el = xmltree::Element::new("fields");

        let mut fields: Vec<_> = r.fields.values().collect();
        fields.sort_by(|a, b| a.range.0.cmp(&b.range.0));
        fields_el.children = fields
            .into_iter()
            .map(svd::field::generate)
            .collect::<Result<Vec<_>, _>>()?;

        el.children.push(fields_el);
    }

    Ok(el)
}
