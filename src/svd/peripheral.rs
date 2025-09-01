use svd_rs::MaybeArray;

use crate::chip;
use crate::svd;
use std::convert::TryInto;

fn create_address_blocks(p: &chip::Peripheral) -> crate::Result<Option<Vec<svd_rs::AddressBlock>>> {
    let mut registers: Vec<&chip::Register> = if p.register_group.is_union {
        if let Some(first_subgroup) = p.register_group.subgroups.first() {
            first_subgroup.registers.values().collect::<Vec<_>>()
        } else {
            log::warn!("Union peripheral {:?} has no subgroups", p.name);
            Vec::new() // Return empty vector if there are no subgroups
        }
    } else {
        p.register_group.registers.values().collect::<Vec<_>>()
    };
    registers.sort_by(|a, b| a.address.cmp(&b.address));

    let new_address_block = |offset: usize, size| {
        svd_rs::AddressBlock::builder()
            .offset(offset as u32)
            .size(size)
            .usage(svd_rs::AddressBlockUsage::Registers)
            .build(svd_rs::ValidateLevel::Strict)
            .map_err(crate::Error::from)
    };

    let mut address_blocks = Vec::new();
    let mut current_offset = registers[0].offset;
    let mut current_size = 0;
    for reg in registers.into_iter() {
        let current_address = current_offset + current_size;
        if current_address == reg.offset {
            current_size += reg.size;
        } else {
            if current_size > 0 {
                address_blocks.push(new_address_block(current_offset, current_size.try_into()?)?);
            }

            current_offset = reg.offset;
            current_size = reg.size;
        }
    }
    address_blocks.push(new_address_block(current_offset, current_size.try_into()?)?);

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

    if register_group.is_union {
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
