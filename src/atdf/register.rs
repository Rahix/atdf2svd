use crate::atdf;
use crate::chip;
use crate::util;
use crate::ElementExt;
use std::collections::BTreeMap;

fn field_map_from_bitfield_children(
    el: &xmltree::Element,
    values: &atdf::values::ValueGroups,
    mode_name: Option<&str>,
) -> crate::Result<BTreeMap<String, chip::Field>> {
    el.children
        .iter()
        .filter_map(|node| node.as_element().filter(|e| e.name == "bitfield"))
        .map(|e| atdf::field::parse(e, values, mode_name))
        .map(|r| r.map(|f| (f.name.clone(), f)))
        .collect::<Result<BTreeMap<_, _>, _>>()
}

pub fn parse(
    el: &xmltree::Element,
    address: usize,
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

    // get bitfield under register
    let mut fields: BTreeMap<String, chip::Field> =
        field_map_from_bitfield_children(el, values, None)?;

    // get bitfield under register.mode
    el.children
        .iter()
        .filter_map(|mode| {
            mode.as_element()
                .filter(|m| m.name == "mode")
                .and_then(|m| m.attributes.get("name").map(|mode_name| (mode_name, m)))
        })
        .try_for_each(|(mode_name, el)| {
            fields.append(&mut field_map_from_bitfield_children(
                el,
                values,
                Some(mode_name),
            )?);
            crate::Result::Ok(())
        })?;

    let offset = util::parse_int(el.attr("offset")?)?;

    Ok(chip::Register {
        name,
        description,
        mode,
        address: address + offset,
        offset,
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

pub fn parse_list(
    register_group_el: &xmltree::Element,
    offset: usize,
    value_groups: &atdf::values::ValueGroups,
) -> crate::Result<BTreeMap<String, chip::Register>> {
    register_group_el
        .iter_children_with_name("register", Some("register-group"))
        .map(|reg| {
            atdf::register::parse(reg, offset, value_groups).map(|r| {
                let key = match r.mode {
                    Some(ref mode) => format!("{mode}_{}", r.name),
                    None => r.name.clone(),
                };
                (key, r)
            })
        })
        .collect()
}
