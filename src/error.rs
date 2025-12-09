pub trait DisplayError {
    fn format(&self, w: &mut dyn std::io::Write) -> std::io::Result<()>;

    fn to_panic(&self) -> ! {
        let mut message: Vec<u8> = "Error: ".into();
        self.format(&mut message).unwrap();
        panic!("{}", String::from_utf8_lossy(&message));
    }
}

pub type Error = Box<dyn DisplayError>;
pub type Result<T> = std::result::Result<T, Error>;

impl<E: DisplayError + 'static> From<E> for Error {
    fn from(e: E) -> Error {
        Box::new(e)
    }
}

pub trait DisplayErrorAuto {}

impl<E: std::fmt::Display + DisplayErrorAuto> DisplayError for E {
    fn format(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "{}", self)
    }
}

impl DisplayErrorAuto for xmltree::ParseError {}
impl DisplayErrorAuto for xmltree::Error {}
impl DisplayErrorAuto for std::io::Error {}
impl DisplayErrorAuto for std::num::ParseIntError {}
impl DisplayErrorAuto for std::num::TryFromIntError {}
impl DisplayErrorAuto for svd_rs::SvdError {}
impl DisplayErrorAuto for svd_encoder::EncodeError {}
