use crate::ElementExt;
use crate::atdf;
use crate::chip;
use crate::util;

pub fn parse_list(
    el: &xmltree::Element,
    modules: &xmltree::Element,
) -> crate::Result<Vec<chip::Peripheral>> {
    let mut peripherals = vec![];

    for module in el.iter_children_with_name("module", None) {
        let module_name = module.attr("name")?;

        for instance in module.iter_children_with_name("instance", Some("module")) {
            let mut registers = vec![];

            // Find corresponding module
            let module = modules.first_child_by_attr(Some("module"), "name", module_name)?;

            // The register definitions can reference value-groups, that are stored on the same
            // level as the register-groups, so we parse them in here first.
            let value_groups = atdf::values::parse_value_groups(module)?;

            for register_group in instance
                .children
                .iter()
                .filter_map(|node| node.as_element().filter(|e| e.name == "register-group"))
            {
                let name = register_group.attr("name-in-module")?;
                let offset = util::parse_int(register_group.attr("offset")?)?;

                let group = module.first_child_by_attr(Some("register-group"), "name", name)?;

                for register in group.iter_children_with_name("register", Some("register-group")) {
                    registers.push(atdf::register::parse(register, offset, &value_groups)?);
                }
            }

            let registers = registers
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
                .collect();

            peripherals.push(chip::Peripheral {
                name: instance.attr("name")?.clone(),
                description: instance
                    .attr("caption")
                    .or(module.attr("caption"))
                    .ok()
                    .cloned()
                    .and_then(|d| if !d.is_empty() { Some(d) } else { None }),
                registers,
            })
        }
    }

    Ok(peripherals)
}
