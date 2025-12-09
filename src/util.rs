use std::mem;

/// Parse an integer from either decimal or hexadecimal
#[expect(clippy::from_str_radix_10)]
pub fn parse_int(s: &str) -> crate::Result<usize> {
    if let Some(hex) = s.strip_prefix("0x") {
        usize::from_str_radix(hex, 16)
    } else {
        usize::from_str_radix(s, 10)
    }
    .map_err(Into::into)
}

/// Parse a bitmask and return the range and whether the full range is covered
pub fn parse_mask(s: &str) -> crate::Result<Option<((usize, usize), bool)>> {
    let mask = parse_int(s)?;
    // An empty mask cannot have a range
    if mask == 0 {
        return Ok(None);
    }

    let bits_set = (0..mem::size_of::<usize>() * 8).filter(|i| (mask & (1 << *i)) > 0);
    let range = (bits_set.clone().min().unwrap(), bits_set.max().unwrap());

    let range_bitmask = ((1 << (range.1 - range.0 + 1)) - 1) << range.0;
    let has_intermediate = range_bitmask ^ mask != 0;

    Ok(Some((range, has_intermediate)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer() {
        assert_eq!(parse_int("0xff").ok(), Some(255));
        assert_eq!(parse_int("23").ok(), Some(23));
        assert!(parse_int("").is_err());
        assert!(parse_int("ff").is_err());
        assert!(parse_int("-7").is_err());
    }
}
