use crate::ElementExt;
use std::fmt;

impl_error! {
    pub struct UnsupportedError {
        pub what: String,
        pub element: String,
    }

    fn fmt(&self, f) {
        write!(f, "{} is unsupported (at {})", self.what, self.element)
    }
}

impl UnsupportedError {
    pub fn new<S: Into<String>>(what: S, el: &xmltree::Element) -> Self {
        UnsupportedError {
            what: what.into(),
            element: el.debug(),
        }
    }
}
