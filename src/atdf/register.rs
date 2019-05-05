use crate::chip;
use crate::util;
use crate::ElementExt;

pub fn parse(el: &xmltree::Element, offset: usize) -> crate::Result<chip::Register> {
    let name = el.attr("name")?.clone();

    let description = el
        .attributes
        .get("caption")
        .and_then(|d| if d.len() != 0 { Some(d) } else { None })
        .cloned();

    let access = if let Some(access) = el.attributes.get("ocd-rw") {
        match access.as_ref() {
            "R" => chip::AccessMode::ReadOnly,
            "" => {
                log::warn!("empty access-mode on {}", el.debug());
                chip::AccessMode::ReadWrite
            }
            _ => panic!("unknown access mode {:?}", access),
        }
    } else {
        chip::AccessMode::ReadWrite
    };

    Ok(chip::Register {
        name,
        description,
        address: util::parse_int(el.attr("offset")?)? + offset,
        access,
    })
}
