use crate::chip;
use crate::svd::restriction::generate_access;

pub fn generate(r: &chip::Register) -> crate::Result<svd_rs::Register> {
    let (write_constraint, _) =
        crate::svd::restriction::generate(&r.restriction, r.size as u32 * 8)?;

    let register = svd_rs::RegisterInfo::builder()
        .name(r.name.clone())
        .description(r.description.clone().or_else(|| {
            log::warn!("Description missing for register \"{}\"", r.name);
            Some("No Description.".to_owned())
        }))
        .address_offset(r.offset as u32)
        .size(if r.size != 0 {
            Some(r.size as u32 * 8)
        } else {
            None
        })
        .access(generate_access(r.access))
        .write_constraint(write_constraint)
        .alternate_group(r.mode.clone());

    let mut fields = r.fields.values().collect::<Vec<_>>();
    fields.sort_by(|a, b| a.range.0.cmp(&b.range.0));

    let fields = fields
        .into_iter()
        .map(crate::svd::field::generate)
        .collect::<Result<Vec<_>, _>>()?;

    register
        .fields(if !fields.is_empty() {
            Some(fields)
        } else {
            None
        })
        .build(svd_rs::ValidateLevel::Strict)
        .map(svd_rs::Register::Single)
        .map_err(crate::Error::from)
}
