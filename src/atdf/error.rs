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

pub struct RecursiveRegisterGroupError {
    group_name: String,
}

impl crate::DisplayError for RecursiveRegisterGroupError {
    fn format(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(
            w,
            "{}: Recursive register group reference detected leading to {}",
            "Error".red().bold(),
            self.group_name.dimmed()
        )
    }
}

impl RecursiveRegisterGroupError {
    pub fn new<S: Into<String>>(group_name: S) -> Self {
        let group_name = group_name.into();

        RecursiveRegisterGroupError { group_name }
    }
}
