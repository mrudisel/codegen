use std::fmt::{self, Write};


use crate::bounds::{Bound, BoundEndsWith};
use crate::docs::Docs;
use crate::formatter::Formatter;

use crate::type_def::Type;

use crate::impl_macros::{
    impl_bound_methods,
    impl_doc_methods,
};

/// Defines an associated type.
#[derive(Debug, Clone)]
pub struct AssociatedType {
    bound: Bound,
    docs: Docs,
}

impl AssociatedType {
    /// Creates a new associated type with no bounds.
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            docs: Docs::default(),
            bound: Bound::new(name),
        }
    }

    /// Creates a new associated type with bounds.
    pub fn new_with_bound<S, T>(name: S, bound: T) -> Self
    where
        S: AsRef<str>,
        T: Into<Type>
    {
        Self {
            docs: Docs::default(),
            bound: Bound::new_with_bound(name, bound),
        }
    }

    pub(crate) fn fmt_assoc_type(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "type ")?;
        self.bound.fmt_bound(formatter, BoundEndsWith::SemiColon)
    }

    impl_bound_methods!(bound);
    impl_doc_methods!(docs);
}
