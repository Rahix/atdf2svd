use std::collections::HashMap;
use crate::chip;
use crate::util;
use crate::ElementExt;

pub type ValueGroups = HashMap<String, HashMap<String, chip::EnumeratedValue>>;

pub fn parse_value_groups (
    module_el: &xmltree::Element
) -> crate::Result<ValueGroups> {
    // Structure: <value-group>
    //                <value />
    //                ...
    //            </value-group>

    let mut value_groups = HashMap::new();
    for value_group_el in module_el
        .children
        .iter()
        .filter(|m| m.name == "value-group")
    {
        let group_name = value_group_el.attr("name")?.clone();

        let mut enumerated_values = HashMap::new();
        for value_el in value_group_el.children.iter() {
            value_el.check_name("value")?;

            let name = value_el.attr("name")?.clone();
            let description = value_el.attributes.get("caption").cloned();
            let value = util::parse_int(value_el.attr("value")?)?;

            enumerated_values.insert(
                name.clone(),
                chip::EnumeratedValue {
                    name, description, value
                }
            );
        }

        value_groups.insert(group_name, enumerated_values);
    }

    Ok(value_groups)
}
