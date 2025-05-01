use crate::atdf;
use crate::chip;
use crate::util;
use crate::ElementExt;
use std::collections::BTreeMap;

pub fn parse_list(
    module_el: &xmltree::Element,
    address: usize,
    value_groups: &atdf::values::ValueGroups,
) -> crate::Result<BTreeMap<String, chip::RegisterGroup>> {
    let mut register_group_headers = BTreeMap::new();
    for register_group_header_el in module_el
        .children
        .iter()
        .filter_map(|node| node.as_element().filter(|e| e.name == "register-group"))
    {
        let name = register_group_header_el.attr("name")?.clone();
        let description = register_group_header_el
            .attributes
            .get("caption")
            .filter(|d| !d.is_empty())
            .cloned();

        let references = parse_references(register_group_header_el)?;
        let registers =
            atdf::register::parse_list(register_group_header_el, address, value_groups)?;

        register_group_headers.insert(
            name.clone(),
            chip::RegisterGroup {
                name,
                description,
                offset: 0,
                registers,
                references,
                // subgroups will be filled in when all register-groups are parsed
                subgroups: vec![],
            },
        );
    }
    Ok(register_group_headers)
}

pub fn build_register_group_hierarchy(
    register_group: &mut chip::RegisterGroup,
    register_groups: &mut BTreeMap<String, chip::RegisterGroup>,
    current_address: usize,
    level: usize,
) -> crate::Result<()> {
    // Infinite recursion guard
    if level > 20 {
        return Err(
            atdf::error::RecursiveRegisterGroupError::new(register_group.name.clone()).into(),
        );
    }
    for reference in register_group.references.iter() {
        if let Some(name_in_module) = &reference.name_in_module {
            if let Some(subgroup) = register_groups.get(name_in_module) {
                let mut subgroup = subgroup.clone();
                // The reference offset & name is used to override the subgroup
                let offset = reference.offset.unwrap_or(0);
                let new_address = current_address + offset;

                if offset > 0 {
                    // Adjust each register's address by the calculated adjustment
                    subgroup.registers.iter_mut().for_each(|(_, register)| {
                        register.address = new_address + register.offset;
                    });
                }
                subgroup.name = reference.name.clone();

                build_register_group_hierarchy(
                    &mut subgroup,
                    register_groups,
                    new_address,
                    level + 1,
                )?;
                register_group.subgroups.push(subgroup);
            }
        }
    }

    Ok(())
}

pub fn parse_references(
    register_group_el: &xmltree::Element,
) -> crate::Result<Vec<chip::RegisterGroupReference>> {
    let mut register_group_references = vec![];

    for register_group_item_el in register_group_el
        .children
        .iter()
        .filter_map(|node| node.as_element().filter(|e| e.name == "register-group"))
    {
        let name = register_group_item_el.attr("name")?.clone();

        let name_in_module = register_group_item_el
            .attributes
            .get("name-in-module")
            .filter(|d| !d.is_empty())
            .cloned();

        let offset = register_group_item_el
            .attributes
            .get("offset")
            .filter(|d| !d.is_empty())
            .map(|d| util::parse_int(d))
            .transpose()?;

        register_group_references.push(chip::RegisterGroupReference {
            name,
            name_in_module,
            offset,
        });
    }

    Ok(register_group_references)
}
