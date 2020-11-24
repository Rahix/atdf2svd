use crate::atdf;
use crate::chip;
use crate::util;
use crate::ElementExt;

pub fn parse_list(
    el: &xmltree::Element,
    modules: &xmltree::Element,
) -> crate::Result<Vec<chip::Peripheral>> {
    let mut peripherals = vec![];

    for module in el.children.iter() {
        module.check_name("module")?;
        let module_name = module.attr("name")?;

        for instance in module.children.iter() {
            instance.check_name("instance")?;

            let mut registers = vec![];

            // Find corresponding module
            let module = modules.first_child_by_attr(Some("module"), "name", module_name)?;

            // The register definitions can reference value-groups, that are stored on the same
            // level as the register-groups, so we parse them in here first.
            let value_groups = atdf::values::parse_value_groups(module)?;

            for register_group in instance
                .children
                .iter()
                .filter(|c| c.name == "register-group")
            {
                let name = register_group.attr("name-in-module")?;
                let offset = util::parse_int(register_group.attr("offset")?)?;

                let group = module.first_child_by_attr(Some("register-group"), "name", name)?;

                for register in group
                    .children
                    .iter()
                    .inspect(|e| {
                        if e.name != "register" {
                            log::warn!("Unhandled register node: {:?}", e.debug())
                        }
                    })
                    .filter(|e| e.name == "register")
                {
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
                    .cloned(),
                registers,
            })
        }
    }

    Ok(peripherals)
}
