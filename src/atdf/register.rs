use crate::chip;
use crate::util;
use crate::ElementExt;

pub fn parse(el: &xmltree::Element, offset: usize) -> crate::Result<chip::Register> {
    Ok(chip::Register {
        name: el.attr("name")?.clone(),
        description: Some(el.attr("caption")?.clone()),
        address: util::parse_int(el.attr("offset")?)? + offset,
        access: chip::AccessMode::ReadOnly,
    })
}
