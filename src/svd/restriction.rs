use crate::chip;
use crate::ElementExt;

pub fn generate(restriction: &chip::ValueRestriction, width: usize) -> crate::Result<Vec<xmltree::Element>> {
    Ok(match restriction {
        chip::ValueRestriction::Unsafe => vec![],
        chip::ValueRestriction::Any => {
            if width == 1 {
                vec![]
            } else {
                let mut el = xmltree::Element::new("writeConstraint");
                let mut range = xmltree::Element::new("range");
                range.child_with_text("minimum", 0.to_string());
                range.child_with_text("maximum", (2usize.pow(width as u32)-1).to_string());
                el.children.push(range);
                vec![el]
            }
        },
        chip::ValueRestriction::Range(lo, hi) => {
            let mut el = xmltree::Element::new("writeConstraint");
            let mut range = xmltree::Element::new("range");
            range.child_with_text("minimum", lo.to_string());
            range.child_with_text("maximum", hi.to_string());
            el.children.push(range);
            vec![el]
        },
        chip::ValueRestriction::Enumerated(enumerated) => {
            let mut wc = xmltree::Element::new("writeConstraint");
            wc.child_with_text("useEnumeratedValues", "true");

            let mut values_el = xmltree::Element::new("enumeratedValues");

            let mut values: Vec<_> = enumerated.values().collect();
            values.sort_by(|v1, v2| v1.value.cmp(&v2.value));
            values_el.children = values
                .into_iter()
                .map(generate_enumerated)
                .collect::<Result<_, _>>()?;

            vec![wc, values_el]
        },
    })
}

pub fn generate_enumerated(e: &chip::EnumeratedValue) -> crate::Result<xmltree::Element> {
    let mut el = xmltree::Element::new("enumeratedValue");
    el.child_with_text("name", e.name.as_ref());
    el.child_with_text(
        "description",
        if let Some(ref desc) = e.description {
            desc.as_ref()
        } else {
            log::warn!("Description missing for enumeratedValue {:?}", e.name);
            "<TBD>"
        },
    );
    el.child_with_text("value", e.value.to_string());

    Ok(el)
}

pub fn generate_access(a: &chip::AccessMode) -> crate::Result<Option<xmltree::Element>> {
    Ok(match a {
        chip::AccessMode::ReadOnly => Some("read-only"),
        chip::AccessMode::WriteOnly => Some("write-only"),
        chip::AccessMode::ReadWrite => None,
    }.map(|m| {
        xmltree::Element::new_with_text("access", m)
    }))
}
