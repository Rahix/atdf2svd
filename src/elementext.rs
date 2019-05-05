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

    /// Check the name to be the expected value or error
    fn check_name(&self, name: &str) -> crate::Result<()>;

    /// Create a new element with text content
    fn new_with_text<S: Into<String>>(name: &str, text: S) -> Self;

    /// Create a new child element with text content
    fn child_with_text<S: Into<String>>(&mut self, name: &str, text: S);
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
            .filter(|c| {
                if let Some(n) = name {
                    if n != c.name {
                        return false;
                    }
                }
                c.attributes.get(attr) == Some(&value.into())
            })
            .next()
            .ok_or_else(|| {
                error::MissingElement::new(
                    format!("<{} {}='{}'>", name.unwrap_or("???"), attr, value),
                    self,
                )
                .into()
            })
    }

    fn check_name(&self, name: &str) -> crate::Result<()> {
        if self.name == name {
            Ok(())
        } else {
            Err(error::WrongName::new(name, self).into())
        }
    }

    fn new_with_text<S: Into<String>>(name: &str, text: S) -> Self {
        let mut el = Self::new(name);
        el.text = Some(text.into());
        el
    }

    fn child_with_text<S: Into<String>>(&mut self, name: &str, text: S) {
        let child = Self::new_with_text(name, text);
        self.children.push(child);
    }
}

pub mod error {
    use super::ElementExt;
    use std::fmt;

    impl_error! {
        pub struct MissingAttribute {
            pub attr: String,
            pub element: String,
        }

        fn fmt(&self, f) {
            write!(f, "attribute {:?} missing on {}", self.attr, self.element)
        }
    }

    impl MissingAttribute {
        pub fn new<S: Into<String>>(attr: S, el: &xmltree::Element) -> Self {
            MissingAttribute {
                attr: attr.into(),
                element: el.debug(),
            }
        }
    }

    impl_error! {
        pub struct MissingElement {
            pub name: String,
            pub parent: String,
        }

        fn fmt(&self, f) {
            write!(f, "element {:?} missing in {}", self.name, self.parent)
        }
    }

    impl MissingElement {
        pub fn new<S: Into<String>>(name: S, el: &xmltree::Element) -> Self {
            MissingElement {
                name: name.into(),
                parent: el.debug(),
            }
        }
    }

    impl_error! {
        pub struct WrongName {
            pub expected: String,
            pub element: String,
        }

        fn fmt(&self, f) {
            write!(f, "wrong name: expected {:?} but got {}", self.expected, self.element)
        }
    }

    impl WrongName {
        pub fn new<S: Into<String>>(expected: S, el: &xmltree::Element) -> Self {
            WrongName {
                expected: expected.into(),
                element: el.debug(),
            }
        }
    }
}
