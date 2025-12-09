use crate::ElementExt;
use crate::chip;
use crate::util;
use std::collections::BTreeMap;

pub type ValueGroups = BTreeMap<String, BTreeMap<String, chip::EnumeratedValue>>;

pub fn parse_value_groups(module_el: &xmltree::Element) -> crate::Result<ValueGroups> {
    // Structure: <value-group>
    //                <value />
    //                ...
    //            </value-group>

    let mut value_groups = BTreeMap::new();
    for value_group_el in module_el
        .children
        .iter()
        .filter_map(|node| node.as_element().filter(|e| e.name == "value-group"))
    {
        let group_name = value_group_el.attr("name")?.clone();

        let mut enumerated_values = BTreeMap::new();
        for value_el in value_group_el
            .children
            .iter()
            .filter_map(|node| node.as_element().filter(|e| e.name == "value"))
        {
            let name = value_el.attr("name")?.clone();
            let description = value_el
                .attributes
                .get("caption")
                .and_then(|d| if !d.is_empty() { Some(d) } else { None })
                .cloned();
            let value = util::parse_int(value_el.attr("value")?)?;

            enumerated_values.insert(
                name.clone(),
                chip::EnumeratedValue {
                    name,
                    description,
                    value,
                },
            );
        }

        value_groups.insert(group_name, enumerated_values);
    }

    Ok(value_groups)
}
