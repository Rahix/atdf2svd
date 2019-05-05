pub mod chip;
pub mod error;
pub mod peripheral;
pub mod register;

pub fn parse<R: std::io::Read>(r: R) -> crate::Result<crate::chip::Chip> {
    let tree = xmltree::Element::parse(r)?;

    chip::parse(&tree).into()
}
