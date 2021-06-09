use crate::chip;
use crate::svd;
use crate::ElementExt;

fn create_address_blocks(p: &chip::Peripheral, el: &mut xmltree::Element) -> crate::Result<()> {
    let mut registers: Vec<_> = p.registers.values().collect();
    registers.sort_by(|a, b| a.address.cmp(&b.address));
    let base = p.base_address().expect("no base address");

    let mut add_address_block = |offset, size| {
        let mut address_block = xmltree::Element::new("addressBlock");
        address_block.child_with_text("offset", format!("0x{:X}", offset - base));
        address_block.child_with_text("size", format!("0x{:X}", size));
        address_block.child_with_text("usage", "registers");
        el.children.push(address_block);
    };

    let mut current_offset = registers[0].address;
    let mut current_size = 0;
    for reg in registers.into_iter() {
        let current_address = current_offset + current_size;
        if current_address == reg.address {
            current_size += reg.size;
        } else {
            add_address_block(current_offset, current_size);

            current_offset = reg.address;
            current_size = reg.size;
        }
    }
    add_address_block(current_offset, current_size);

    Ok(())
}

pub fn generate(p: &chip::Peripheral) -> crate::Result<xmltree::Element> {
    let mut el = xmltree::Element::new("peripheral");
    let base = p.base_address().expect("todo error");

    el.child_with_text("name", p.name.clone());
    el.child_with_text(
        "description",
        if let Some(ref desc) = p.description {
            desc.as_ref()
        } else {
            log::warn!("Description missing for peripheral {:?}", p.name);
            "<TBD>"
        },
    );
    el.child_with_text("baseAddress", format!("0x{:X}", base));

    create_address_blocks(p, &mut el)?;

    let mut registers = xmltree::Element::new("registers");

    registers.children = p
        .registers
        .values()
        .map(|r| svd::register::generate(r, base))
        .collect::<Result<Vec<_>, _>>()?;

    el.children.push(registers);

    Ok(el)
}
