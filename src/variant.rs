use std::fmt::{self, Write};

use crate::attributes::Attributes;
use crate::docs::Docs;
use crate::fields::Fields;
use crate::formatter::Formatter;

use crate::type_def::Type;

use crate::impl_macros::{
    impl_attr_methods,
    impl_doc_methods,
};

/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    name: String,
    docs: Docs,
    attrs: Attributes,
    fields: Fields,
}

impl Variant {
    /// Return a new enum variant with the given name.
    pub fn new(name: &str) -> Self {
        Variant {
            name: name.to_string(),
            docs: Docs::default(),
            attrs: Attributes::default(),
            fields: Fields::Empty,
        }
    }

    /// Add a named field to the variant.
    pub fn named<T>(&mut self, name: &str, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    pub fn tuple(&mut self, ty: &str) -> &mut Self {
        self.fields.tuple(ty);
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.docs.fmt_docs(fmt)?;
        self.attrs.fmt_attrs(fmt)?;

        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        write!(fmt, ",\n")?;

        Ok(())
    }

    impl_attr_methods!(attrs);
    impl_doc_methods!(docs);
}
