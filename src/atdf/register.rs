use crate::chip;
use crate::util;
use crate::ElementExt;

pub fn parse(el: &xmltree::Element, offset: usize) -> crate::Result<chip::Register> {
    let description = el
        .attributes
        .get("caption")
        .and_then(|d| if d.len() != 0 { Some(d) } else { None })
        .cloned();

    let access = if let Some(access) = el.attributes.get("ocd-rw") {
        match access.as_ref() {
            "R" => chip::AccessMode::ReadOnly,
            "" => chip::AccessMode::ReadWrite, // TODO Emit a warning
            _ => panic!("unknown access mode {:?}", access),
        }
    } else {
        chip::AccessMode::ReadWrite
    };

    Ok(chip::Register {
        name: el.attr("name")?.clone(),
        description,
        address: util::parse_int(el.attr("offset")?)? + offset,
        access,
    })
}
