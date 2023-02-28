use crate::atdf;
use crate::chip;
use crate::util;
use crate::ElementExt;

pub fn parse_list(
    el: &xmltree::Element,
    modules: &xmltree::Element,
) -> crate::Result<Vec<chip::Peripheral>> {
    let mut peripherals = vec![];

    for module in el.children.iter().filter_map(|node| {
        let module = node.as_element().filter(|e| e.name == "module");

        if module.is_none() {
            log::warn!("Unhandled module element: {module:?}");
        }

        module
    }) {
        let module_name = module.attr("name")?;

        for instance in module.children.iter().filter_map(|node| {
            let instance = node.as_element().filter(|e| e.name == "instance");

            if instance.is_none() {
                log::warn!("Unhandled instance element: {module:?}");
            }

            instance
        }) {
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

                for register in group.children.iter().filter_map(|node| {
                    let register = node.as_element().filter(|e| e.name == "register");

                    if register.is_none() {
                        log::warn!("Unhandled register node: {node:?}");
                    }

                    register
                }) {
                    registers.push(atdf::register::parse(register, offset, &value_groups)?);
                }
            }

            let registers = registers.into_iter().map(|r| (r.name.clone(), r)).collect();

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
