use crate::ElementExt;
use colored::Colorize;

pub struct UnsupportedError(String, String);

impl crate::DisplayError for UnsupportedError {
    fn format(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(
            w,
            "{} is unsupported in element\n    {}",
            self.0,
            self.1.dimmed()
        )
    }
}

impl UnsupportedError {
    pub fn new<S: Into<String>>(what: S, el: &xmltree::Element) -> Self {
        UnsupportedError(what.into(), el.debug())
    }
}
