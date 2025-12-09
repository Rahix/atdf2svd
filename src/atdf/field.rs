use crate::ElementExt;
use crate::atdf;
use crate::chip;
use crate::util;

pub fn parse(
    bitfield_el: &xmltree::Element,
    value_groups: &atdf::values::ValueGroups,
    mode_name: Option<&str>,
) -> crate::Result<chip::Field> {
    debug_assert!(bitfield_el.name == "bitfield");
    let name: String = if let Some(mode) = mode_name {
        format!("{}_{}", mode, bitfield_el.attr("name")?)
    } else {
        bitfield_el.attr("name")?.clone()
    };
    let description = bitfield_el
        .attributes
        .get("caption")
        .and_then(|d| if !d.is_empty() { Some(d) } else { None })
        .cloned();
    let values = bitfield_el.attributes.get("values");

    // The range is defined by a mask.
    // Not that in some cases there are bits withing this range, that do not belong to this mask
    // (e.g. 0b00010010). Then the value restriction is unsafe.
    let mask = bitfield_el.attr("mask")?;
    let (range, unsafe_range) = util::parse_mask(mask)?.ok_or_else(|| {
        atdf::error::UnsupportedError::new(format!("mask {:?}", mask), bitfield_el)
    })?;

    let restriction = if let Some(id) = values {
        let values = value_groups.get(id).ok_or_else(|| {
            crate::elementext::error::MissingElement::new(
                format!("<value-group name=\"{}\" ...>", id),
                bitfield_el,
            )
        })?;
        let mask_as_int = util::parse_int(mask)?;
        let mask_as_int = mask_as_int >> mask_as_int.trailing_zeros();
        let filtered_values: std::collections::BTreeMap<_, _> = values
            .iter()
            .filter(|(_, ev)| ev.value & mask_as_int == ev.value)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        if values.len() != filtered_values.len() {
            log::warn!("Invalid enumerated values dropped for field {}", name);
        }
        if !filtered_values.is_empty() {
            chip::ValueRestriction::Enumerated(filtered_values)
        } else {
            log::warn!("Empty enumerated values for field {}", name);
            chip::ValueRestriction::Unsafe
        }
    } else if unsafe_range {
        chip::ValueRestriction::Unsafe
    } else {
        chip::ValueRestriction::Any
    };

    let access = if let Some(access) = bitfield_el.attributes.get("rw") {
        match access.as_ref() {
            "R" => chip::AccessMode::ReadOnly,
            "RW" => chip::AccessMode::ReadWrite,
            "W" => chip::AccessMode::WriteOnly,
            "" => {
                log::warn!("empty access-mode on {:?}", bitfield_el);
                chip::AccessMode::ReadWrite
            }
            _ => {
                return Err(atdf::error::UnsupportedError::new(
                    format!("access-mode '{:?}'", access),
                    bitfield_el,
                )
                .into());
            }
        }
    } else {
        chip::AccessMode::ReadWrite
    };

    Ok(chip::Field {
        name,
        description,
        range,
        access,
        restriction,
    })
}
