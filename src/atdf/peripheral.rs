use crate::atdf;
use crate::chip;
use crate::util;
use crate::ElementExt;

pub fn parse_list(
    el: &xmltree::Element,
    modules: &xmltree::Element,
) -> crate::Result<Vec<chip::Peripheral>> {
    let mut peripherals = vec![];

    for module in el.iter_children_with_name("module", None) {
        let module_name = module.attr("name")?;

        for instance in module.iter_children_with_name("instance", Some("module")) {
            // Find corresponding module
            let module = modules.first_child_by_attr(Some("module"), "name", module_name)?;

            // The register definitions can reference value-groups, that are stored on the same
            // level as the register-groups, so we parse them in here first.
            let value_groups = atdf::values::parse_value_groups(module)?;

            // An instance should always have one register-group
            let instance_register_group = match instance.first_child("register-group") {
                Ok(rg) => rg,
                Err(_) => continue,
            };
            let name_in_module = instance_register_group.attr("name-in-module")?;
            let offset = util::parse_int(instance_register_group.attr("offset")?)?;
            let register_group_headers =
                atdf::register::parse_register_group_headers(module, offset, &value_groups)?;
            let main_register_group = register_group_headers.get(name_in_module).cloned().unwrap();
            let registers = main_register_group.registers;

            peripherals.push(chip::Peripheral {
                name: instance.attr("name")?.clone(),
                name_in_module: name_in_module.clone(),
                description: instance
                    .attr("caption")
                    .or(module.attr("caption"))
                    .ok()
                    .cloned()
                    .and_then(|d| if !d.is_empty() { Some(d) } else { None }),
                registers,
                register_group_headers: register_group_headers.clone(),
            })
        }
    }

    Ok(peripherals)
}
