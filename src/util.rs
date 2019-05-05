/// Parse an integer from either decimal or hexadecimal
pub fn parse_int(s: &str) -> crate::Result<usize> {
    if s.starts_with("0x") {
        usize::from_str_radix(&s[2..], 16)
    } else {
        usize::from_str_radix(s, 10)
    }
    .map_err(Into::into)
}
