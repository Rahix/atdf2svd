use crate::chip;
use std::convert::TryInto;

pub fn generate(
    restriction: &chip::ValueRestriction,
    width: u32,
) -> crate::Result<(
    Option<svd_rs::WriteConstraint>,
    Vec<svd_rs::EnumeratedValues>,
)> {
    let restrictions = match restriction {
        // No write constraint, the constraint is specified by the size of the
        // field in bits.
        //
        // A software that generates register access from this should generate
        // a correct bit mask.
        chip::ValueRestriction::Any => (
            if width > 1 {
                Some(svd_rs::WriteConstraint::Range(
                    svd_rs::WriteConstraintRange {
                        min: 0,
                        max: 2u64.pow(width) - 1,
                    },
                ))
            } else {
                None
            },
            vec![],
        ),
        chip::ValueRestriction::Range(ref lo, ref hi) => (
            Some(svd_rs::WriteConstraint::Range(
                svd_rs::WriteConstraintRange { min: *lo, max: *hi },
            )),
            vec![],
        ),
        chip::ValueRestriction::Enumerated(ref v) => {
            let mut values = v.values().collect::<Vec<_>>();
            values.sort_by(|a, b| a.value.cmp(&b.value));

            let values = values
                .into_iter()
                .map(generate_enumerated)
                .collect::<Result<Vec<_>, _>>()?;

            let enumerated_values = svd_rs::EnumeratedValues::builder()
                .values(values)
                .build(svd_rs::ValidateLevel::Strict)?;

            (
                Some(svd_rs::WriteConstraint::UseEnumeratedValues(true)),
                vec![enumerated_values],
            )
        }
        _ => (None, vec![]),
    };

    Ok(restrictions)
}

pub fn generate_enumerated(e: &chip::EnumeratedValue) -> crate::Result<svd_rs::EnumeratedValue> {
    svd_rs::EnumeratedValue::builder()
        .name(e.name.clone())
        .description(e.description.clone().or_else(|| {
            log::warn!("Description missing for enumeratedValue {:?}", e.name);
            Some("No Description.".to_owned())
        }))
        .value(Some(e.value.try_into()?))
        .build(svd_rs::ValidateLevel::Strict)
        .map_err(crate::Error::from)
}

pub fn generate_access(a: chip::AccessMode) -> Option<svd_rs::Access> {
    match a {
        chip::AccessMode::ReadOnly => Some(svd_rs::Access::ReadOnly),
        chip::AccessMode::WriteOnly => Some(svd_rs::Access::WriteOnly),
        chip::AccessMode::ReadWrite => Some(svd_rs::Access::ReadWrite),
        chip::AccessMode::NoAccess => None,
    }
}
