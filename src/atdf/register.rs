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

pub fn parse_list(
    register_group_header_el: &xmltree::Element,
    offset: usize,
    value_groups: &atdf::values::ValueGroups,
) -> crate::Result<BTreeMap<String, chip::Register>> {
    let mut registers = vec![];

    for register in
        register_group_header_el.iter_children_with_name("register", Some("register-group"))
    {
        registers.push(atdf::register::parse(register, offset, value_groups)?);
    }
    Ok(registers
        .into_iter()
        .map(|r| {
            (
                match r.mode {
                    Some(ref mode) => format!("{mode}_{}", r.name),
                    _ => r.name.clone(),
                },
                r,
            )
        })
        .collect())
}

pub fn parse_register_group_headers(
    module_el: &xmltree::Element,
    offset: usize,
    value_groups: &atdf::values::ValueGroups,
) -> crate::Result<BTreeMap<String, chip::RegisterGroupHeader>> {
    let mut register_group_headers = BTreeMap::new();
    for register_group_header_el in module_el
        .children
        .iter()
        .filter_map(|node| node.as_element().filter(|e| e.name == "register-group"))
    {
        let name = register_group_header_el.attr("name")?.clone();
        let class = register_group_header_el
            .attributes
            .get("class")
            .and_then(|d| if !d.is_empty() { Some(d) } else { None })
            .cloned();
        let description = register_group_header_el
            .attributes
            .get("caption")
            .and_then(|d| if !d.is_empty() { Some(d) } else { None })
            .cloned();

        let size = register_group_header_el
            .attributes
            .get("size")
            .and_then(|d| {
                if !d.is_empty() {
                    util::parse_int(d).ok()
                } else {
                    None
                }
            });

        let register_group_items = parse_register_group_items(register_group_header_el)?;
        let registers = parse_list(register_group_header_el, offset, value_groups)?;

        register_group_headers.insert(
            name.clone(),
            chip::RegisterGroupHeader {
                name,
                class,
                description,
                size,
                register_group_items,
                registers,
            },
        );
    }
    Ok(register_group_headers)
}

pub fn parse_register_group_items(
    register_group_header_el: &xmltree::Element,
) -> crate::Result<BTreeMap<String, chip::RegisterGroupItem>> {
    let mut register_group_items = BTreeMap::new();

    for register_group_item_el in register_group_header_el
        .children
        .iter()
        .filter_map(|node| node.as_element().filter(|e| e.name == "register-group"))
    {
        let name = register_group_item_el.attr("name")?.clone();
        let description = register_group_item_el
            .attributes
            .get("caption")
            .and_then(|d| if !d.is_empty() { Some(d) } else { None })
            .cloned();

        let name_in_module = register_group_item_el
            .attributes
            .get("name-in-module")
            .and_then(|d| if !d.is_empty() { Some(d) } else { None })
            .cloned();

        let size = register_group_item_el.attributes.get("size").and_then(|d| {
            if !d.is_empty() {
                util::parse_int(d).ok()
            } else {
                None
            }
        });

        let offset = register_group_item_el
            .attributes
            .get("offset")
            .and_then(|d| {
                if !d.is_empty() {
                    util::parse_int(d).ok()
                } else {
                    None
                }
            });

        let count = register_group_item_el
            .attributes
            .get("count")
            .and_then(|d| {
                if !d.is_empty() {
                    util::parse_int(d).ok()
                } else {
                    None
                }
            });

        register_group_items.insert(
            name.clone(),
            chip::RegisterGroupItem {
                name,
                name_in_module,
                description,
                size,
                offset,
                count,
            },
        );
    }

    Ok(register_group_items)
}
