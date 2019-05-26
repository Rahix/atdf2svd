use crate::chip;
use crate::util;
use crate::ElementExt;

pub fn parse(interrupt: &xmltree::Element) -> crate::Result<chip::Interrupt> {
    interrupt.check_name("interrupt")?;

    let name = interrupt.attr("name")?.clone();
    let index = util::parse_int(interrupt.attr("index")?)?;
    let description = interrupt
        .attributes
        .get("caption")
        .and_then(|d| if d.len() != 0 { Some(d) } else { None })
        .cloned();

    Ok(chip::Interrupt {
        name,
        description,
        index,
    })
}
