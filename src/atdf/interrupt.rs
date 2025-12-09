use crate::ElementExt;
use crate::chip;
use crate::util;

pub fn parse(interrupt: &xmltree::Element) -> crate::Result<chip::Interrupt> {
    debug_assert!(interrupt.name == "interrupt");

    let name = {
        let inst_name = interrupt.attr("name")?;
        interrupt
            .attr("module-instance")
            .map_or_else(|_| inst_name.clone(), |s| format!("{}_{}", s, inst_name))
    };
    let index = util::parse_int(interrupt.attr("index")?)?;
    let description = interrupt
        .attributes
        .get("caption")
        .and_then(|d| if !d.is_empty() { Some(d) } else { None })
        .cloned();

    Ok(chip::Interrupt {
        name,
        description,
        index,
    })
}
