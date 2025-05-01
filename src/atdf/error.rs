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
