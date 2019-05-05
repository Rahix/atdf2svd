//! Extensions to the xmltree::Element for convenience

pub trait ElementExt {
    fn debug(&self) -> String;
    fn attr(&self, name: &str) -> crate::Result<&String>;
    fn first_child(&self, name: &str) -> crate::Result<&Self>;
    fn first_child_by_attr(
        &self,
        name: Option<&str>,
        attr: &str,
        value: &str,
    ) -> crate::Result<&Self>;

    fn check_name(&self, name: &str) -> crate::Result<()>;
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
