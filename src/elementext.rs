//! Extensions to the xmltree::Element for convenience

/// Extensions to the xmltree::Element for convenience
pub trait ElementExt {
    /// Create a debug representation of this element
    fn debug(&self) -> String;

    /// Get an attributes' value or error
    fn attr(&self, name: &str) -> crate::Result<&String>;

    /// Get the first child with a certain name or error
    fn first_child(&self, name: &str) -> crate::Result<&Self>;

    /// Get the first child with a certain attribute set
    ///
    /// You can optionally specify a name the element should have.
    fn first_child_by_attr(
        &self,
        name: Option<&str>,
        attr: &str,
        value: &str,
    ) -> crate::Result<&Self>;
}

impl ElementExt for xmltree::Element {
    fn debug(&self) -> String {
        use std::fmt::Write;

        let mut s = "<".to_string();
        if let Some(ref prefix) = self.prefix {
            write!(&mut s, "{}:", prefix).unwrap();
        }
        write!(&mut s, "{} ", self.name).unwrap();

        for (attr, value) in self.attributes.iter() {
            write!(&mut s, "{}={:?} ", attr, value).unwrap();
        }

        write!(&mut s, "...>").unwrap();

        s
    }

    fn attr(&self, name: &str) -> crate::Result<&String> {
        self.attributes
            .get(name)
            .ok_or_else(|| error::MissingAttribute::new(name, self).into())
    }

    fn first_child(&self, name: &str) -> crate::Result<&Self> {
        self.get_child(name)
            .ok_or_else(|| error::MissingElement::new(name, self).into())
    }

    fn first_child_by_attr(
        &self,
        name: Option<&str>,
        attr: &str,
        value: &str,
    ) -> crate::Result<&Self> {
        self.children
            .iter()
            .filter_map(|node| node.as_element())
            .find(|c| {
                if let Some(n) = name {
                    if n != c.name {
                        return false;
                    }
                }
                c.attributes.get(attr) == Some(&value.into())
            })
            .ok_or_else(|| {
                error::MissingElement::new(
                    format!("<{} {}='{}'>", name.unwrap_or("???"), attr, value),
                    self,
                )
                .into()
            })
    }
}

pub mod error {
    use super::ElementExt;
    use colored::Colorize;

    pub struct MissingAttribute(String, String);

    impl crate::DisplayError for MissingAttribute {
        fn format(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(
                w,
                "Missing attribute {:?} on\n   {}",
                self.0,
                self.1.dimmed()
            )
        }
    }

    impl MissingAttribute {
        pub fn new<S: Into<String>>(attr: S, el: &xmltree::Element) -> Self {
            MissingAttribute(attr.into(), el.debug())
        }
    }

    pub struct MissingElement(String, String);

    impl crate::DisplayError for MissingElement {
        fn format(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(w, "Missing child {:?} in\n   {}", self.0, self.1.dimmed())
        }
    }

    impl MissingElement {
        pub fn new<S: Into<String>>(name: S, el: &xmltree::Element) -> Self {
            MissingElement(name.into(), el.debug())
        }
    }

    pub struct WrongName(String, String);

    impl crate::DisplayError for WrongName {
        fn format(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(w, "Expected {:?} but got\n   {}", self.0, self.1.dimmed())
        }
    }

    impl WrongName {
        pub fn new<S: Into<String>>(name: S, el: &xmltree::Element) -> Self {
            WrongName(name.into(), el.debug())
        }
    }
}
