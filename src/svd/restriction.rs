use crate::chip;
use crate::ElementExt;

pub fn generate(restriction: &chip::ValueRestriction, width: usize) -> crate::Result<Vec<xmltree::Element>> {
    Ok(match restriction {
        chip::ValueRestriction::Unsafe => vec![],
        chip::ValueRestriction::Any => {
            let mut el = xmltree::Element::new("writeConstraint");
            let mut range = xmltree::Element::new("range");
            range.child_with_text("minimum", 0.to_string());
            range.child_with_text("maximum", (2usize.pow(width as u32)-1).to_string());
            el.children.push(range);
            vec![el]
        },
        chip::ValueRestriction::Range(lo, hi) => {
            let mut el = xmltree::Element::new("writeConstraint");
            let mut range = xmltree::Element::new("range");
            range.child_with_text("minimum", lo.to_string());
            range.child_with_text("maximum", hi.to_string());
            el.children.push(range);
            vec![el]
        },
        _ => unimplemented!(),
    })
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
