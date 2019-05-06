use std::collections::HashMap;
use crate::chip;
use crate::util;
use crate::atdf;
use crate::ElementExt;

pub fn parse(el: &xmltree::Element, offset: usize, values: &atdf::values::ValueGroups) -> crate::Result<chip::Register> {
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

    let fields: HashMap<String, chip::Field> = el.children.iter()
        .filter(|c| c.name == "bitfield")
        .map(|e| atdf::field::parse(e, values))
        .map(|r| r.map(|f| (f.name.clone(), f)))
        .collect::<Result<HashMap<_, _>, _>>()?;

    Ok(chip::Register {
        name,
        description,
        address: util::parse_int(el.attr("offset")?)? + offset,
        size: util::parse_int(el.attr("size")?)?,
        access,
        restriction: chip::ValueRestriction::Unsafe,
        fields,
    })
}
