use crate::ElementExt;

pub struct UnsupportedError(String, String);

impl crate::DisplayError for UnsupportedError {
    fn format(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        let maybe_styled_element = {
            cfg_if::cfg_if! {
                if #[cfg(feature = "cli")] {
                    use colored::Colorize;
                    self.1.dimmed()
                } else {
                    &self.1
                }
            }
        };
        write!(
            w,
            "{} is unsupported in element\n    {}",
            self.0, maybe_styled_element,
        )
    }
}

impl UnsupportedError {
    pub fn new<S: Into<String>>(what: S, el: &xmltree::Element) -> Self {
        UnsupportedError(what.into(), el.debug())
    }
}
