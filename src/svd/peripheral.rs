use svd_rs::MaybeArray;

use crate::chip;
use crate::svd;
use std::convert::TryInto;

fn create_address_blocks(p: &chip::Peripheral) -> crate::Result<Option<Vec<svd_rs::AddressBlock>>> {
    let mut registers = p.register_group.get_all_registers().clone();
    registers.sort_by(|a, b| a.address.cmp(&b.address));

    let new_address_block = |offset: usize, size, reg: Option<&chip::Register>| {
        // Calculate the offset relative to the peripheral's base address
        // Handle the case where offset might be less than p.address
        let offset_value = if offset >= p.address {
            offset - p.address
        } else {
            let fd = if let Some(reg) = reg {
                reg.name.clone()
            } else {
                "Unknown".to_string()
            };
            println!("cargo:warning=Register address {} ({:#x}) is less than peripheral base address ({:#x})", fd, offset, p.address);
            0 // Default to 0 if we have an underflow situation
        };
        let offset = offset_value.try_into().map_err(crate::Error::from)?;

        svd_rs::AddressBlock::builder()
            .offset(offset)
            .size(size)
            .usage(svd_rs::AddressBlockUsage::Registers)
            .build(svd_rs::ValidateLevel::Strict)
            .map_err(crate::Error::from)
    };

    let mut address_blocks = Vec::new();
    let mut current_offset = registers[0].address;
    let mut current_size = 0;
    for reg in registers.into_iter() {
        let current_address = current_offset + current_size;
        if current_address == reg.address {
            current_size += reg.size;
        } else {
            address_blocks.push(new_address_block(
                current_offset,
                current_size.try_into()?,
                Some(reg),
            )?);

            current_offset = reg.address;
            current_size = reg.size;
        }
    }
    address_blocks.push(new_address_block(
        current_offset,
        current_size.try_into()?,
        None,
    )?);

    let address_blocks = if !address_blocks.is_empty() {
        Some(address_blocks)
    } else {
        None
    };

    Ok(address_blocks)
}

pub fn create_register_clusters(
    register_group: &chip::RegisterGroup,
) -> crate::Result<Vec<svd_rs::RegisterCluster>> {
    let mut result: Vec<svd_rs::RegisterCluster> = vec![];

    for register in register_group.registers.values() {
        let register_cluster =
            svd::register::generate(register).map(svd_rs::RegisterCluster::Register)?;
        result.push(register_cluster);
    }

    for subgroup in register_group.subgroups.iter() {
        let subgroup_clusters = create_register_clusters(subgroup)?;
        let cluster = svd_rs::RegisterCluster::Cluster(MaybeArray::Single(
            svd_rs::ClusterInfo::builder()
                .name(subgroup.name.clone())
                .description(subgroup.description.clone())
                .address_offset(subgroup.offset as u32)
                .children(subgroup_clusters)
                .build(svd_rs::ValidateLevel::Strict)
                .map_err(crate::Error::from)?,
        ));
        result.push(cluster);
    }

    Ok(result)
}

pub fn generate(p: &chip::Peripheral) -> crate::Result<svd_rs::Peripheral> {
    svd_rs::PeripheralInfo::builder()
        .name(p.name.clone())
        .description(p.description.clone().or_else(|| {
            log::warn!("Description missing for peripheral {:?}", p.name);
            Some("No Description.".to_owned())
        }))
        .base_address(u64::from(p.address as u32))
        .address_block(create_address_blocks(p)?)
        .registers(Some(create_register_clusters(&p.register_group)?))
        .build(svd_rs::ValidateLevel::Strict)
        .map(svd_rs::Peripheral::Single)
        .map_err(crate::Error::from)
}
