use crate::atdf;
use crate::chip;
use crate::util;
use crate::ElementExt;
use std::collections::BTreeMap;

pub fn parse(
    el: &xmltree::Element,
    offset: usize,
    values: &atdf::values::ValueGroups,
) -> crate::Result<chip::Register> {
    let name = el.attr("name")?.clone();

    let description = el
        .attributes
        .get("caption")
        .and_then(|d| if !d.is_empty() { Some(d) } else { None })
        .cloned();

    let mode = el
        .attributes
        .get("modes")
        .and_then(|d| if !d.is_empty() { Some(d) } else { None })
        .cloned();

    let access = if let Some(access) = el.attributes.get("rw") {
        match access.as_ref() {
            "" => chip::AccessMode::NoAccess,
            "R" => chip::AccessMode::ReadOnly,
            "W" => chip::AccessMode::WriteOnly,
            "RW" => chip::AccessMode::ReadWrite,
            _ => chip::AccessMode::ReadWrite,
        }
    } else {
        chip::AccessMode::ReadWrite
    };

    let mut fields: BTreeMap<String, chip::Field> = el
        .children
        .iter()
        .filter_map(|node| node.as_element().filter(|e| e.name == "bitfield"))
        .map(|e| atdf::field::parse(e, values, None))
        .map(|r| r.map(|f| (f.name.clone(), f)))
        .collect::<Result<BTreeMap<_, _>, _>>()?;

    let modes = el
        .children
        .iter()
        .filter_map(|mode| mode.as_element().filter(|m| m.name == "mode"))
        .into_iter();
    for mode in modes {
        if let Some(mode_name) = mode.attributes.get("name").cloned() {
            let res = mode
                .children
                .clone()
                .into_iter()
                .filter_map(|el| el.as_element().filter(|e| e.name == "bitfield").cloned())
                .map(|e| atdf::field::parse(&e, values, Some(mode_name.clone())))
                .map(|r| r.map(|f| (f.name.clone(), f)))
                .collect::<Result<BTreeMap<_, _>, _>>()?;
            fields.append(&mut res.clone());
        }
    }
    Ok(chip::Register {
        name,
        description,
        mode,
        address: util::parse_int(el.attr("offset")?)? + offset,
        size: util::parse_int(el.attr("size")?)?,
        access,
        restriction: if fields.is_empty() {
            chip::ValueRestriction::Any
        } else {
            chip::ValueRestriction::Unsafe
        },
        fields,
    })
}
