/// General Error Type/Trait
pub trait Error: std::error::Error {}

/// Convenience Result Wrapper
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

impl<T: Error + 'static> From<T> for Box<dyn Error> {
    fn from(e: T) -> Box<dyn Error> {
        Box::new(e)
    }
}

impl Error for xmltree::ParseError {}
impl Error for xmltree::Error {}
impl Error for std::io::Error {}
impl Error for std::num::ParseIntError {}

#[macro_export]
macro_rules! impl_error {
    (pub struct $name:ident {
        $(pub $field:ident: $type:ty,)+
    }

    fn fmt(&$self:ident, $fmt:ident) $write:block) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $field: $type,)+
        }

        impl fmt::Display for $name {
            fn fmt(&$self, $fmt: &mut fmt::Formatter) -> fmt::Result $write
        }

        impl std::error::Error for $name {}
        impl crate::Error for $name {}
    }
}
