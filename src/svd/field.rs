use crate::chip;
use crate::svd;
use std::convert::TryInto;

pub fn generate(f: &chip::Field) -> crate::Result<svd_rs::Field> {
    let (write_constraint, enumerated_values) =
        svd::restriction::generate(&f.restriction, f.width().try_into()?)?;
    let (lsb, msb) = (f.range.0 as u32, f.range.1 as u32);

    let field = svd_rs::FieldInfo::builder()
        .name(f.name.clone())
        .description(f.description.clone().or_else(|| {
            log::warn!("Description missing for field {:?}", f.name);
            Some("No Description.".to_owned())
        }))
        .bit_range(svd_rs::BitRange {
            offset: lsb,
            width: msb - lsb + 1,
            range_type: svd_rs::BitRangeType::BitRange,
        })
        .access(svd::restriction::generate_access(f.access))
        .write_constraint(write_constraint)
        .enumerated_values(enumerated_values)
        .build(svd_rs::ValidateLevel::Strict)
        .map(svd_rs::Field::Single)?;

    Ok(field)
}
