use svd_rs::MaybeArray;

use crate::chip;
use crate::svd;
use std::convert::TryInto;

fn create_address_blocks(
    p: &chip::Peripheral,
    base: usize,
) -> crate::Result<Option<Vec<svd_rs::AddressBlock>>> {
    let mut registers: Vec<_> = match p.is_union() {
        true => {
            let &(header, _) = p
                .get_union_register_group_headers()
                .first()
                .expect("No union header found");
            header.registers.values().collect()
        }
        false => p.registers.values().collect(),
    };
    registers.sort_by(|a, b| a.address.cmp(&b.address));

    let new_address_block = |offset: usize, size| {
        let offset = (offset - base).try_into().map_err(crate::Error::from)?;

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
            address_blocks.push(new_address_block(current_offset, current_size.try_into()?)?);

            current_offset = reg.address;
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

pub fn generate(p: &chip::Peripheral) -> crate::Result<svd_rs::Peripheral> {
    let base: u32 = p
        .base_address()
        .expect("Could not retrieve peripheral base address")
        .try_into()?;

    let registers: Vec<svd_rs::RegisterCluster> = match p.is_union() {
        true => {
            println!("cargo:warning=svd generate union style {}", p.name);
            p.get_union_register_group_headers()
                .iter()
                .map(|(header, item)| {
                    let cluster = svd_rs::ClusterInfo::builder()
                        .name(header.name.clone())
                        .description(header.description.clone())
                        .address_offset(item.offset.unwrap_or(0) as u32)
                        .children(
                            header
                                .registers
                                .values()
                                .map(|r| {
                                    svd::register::generate(r, base)
                                        .map(svd_rs::RegisterCluster::Register)
                                })
                                .collect::<Result<Vec<_>, _>>()?,
                        );

                    cluster
                        .build(svd_rs::ValidateLevel::Strict)
                        .map(|cluster_info| {
                            svd_rs::RegisterCluster::Cluster(MaybeArray::Single(cluster_info))
                        })
                        .map_err(crate::Error::from)
                })
                .collect::<Result<Vec<_>, _>>()?
        }
        false => p
            .registers
            .values()
            .map(|r| svd::register::generate(r, base).map(svd_rs::RegisterCluster::Register))
            .collect::<Result<Vec<_>, _>>()?,
    };

    svd_rs::PeripheralInfo::builder()
        .name(p.name.clone())
        .description(p.description.clone().or_else(|| {
            log::warn!("Description missing for peripheral {:?}", p.name);
            Some("No Description.".to_owned())
        }))
        .base_address(u64::from(base))
        .address_block(create_address_blocks(p, base as usize)?)
        .registers(Some(registers))
        .build(svd_rs::ValidateLevel::Strict)
        .map(svd_rs::Peripheral::Single)
        .map_err(crate::Error::from)
}
