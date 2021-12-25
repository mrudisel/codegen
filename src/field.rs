use std::fmt::{self, Write};

use crate::attributes::Attributes;
use crate::docs::Docs;
use crate::formatter::Formatter;
use crate::vis::Vis;

use crate::type_def::Type;

use crate::impl_macros::{
    impl_attr_methods,
    impl_doc_methods,
    impl_ty_methods,
    impl_vis_methods,
};

/// Defines a struct field.
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name
    name: Option<String>,
    /// Field type
    ty: Type,
    /// Field documentation
    docs: Docs,
    /// Field attrs
    attrs: Attributes,
    /// field visibility
    vis: Vis,

}

impl Field {
    /// Return a field definition with the provided name and type
    pub fn new_named<S, T>(name: S, ty: T) -> Self
    where
        S: AsRef<str>,
        T: Into<Type>,
    {
        Self {
            name: Some(name.as_ref().to_owned()),
            ty: ty.into(),
            docs: Docs::default(),
            attrs: Attributes::default(),
            vis: Vis::Private,
        }
    }

    /// Creates a new unnamed field.
    pub fn new_unnamed<T>(ty: T) -> Self
    where
        T: Into<Type>,
    {
        Self {
            name: None,
            ty: ty.into(),
            docs: Docs::default(),
            attrs: Attributes::default(),
            vis: Vis::Private,
        }
    }

    /// Whether or not this field is a named field
    pub fn is_named(&self) -> bool {
        self.name.is_some()
    }

    pub(crate) fn fmt_field(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.docs.fmt_docs(formatter)?;
        self.attrs.fmt_attrs(formatter)?;

        if let Some(name) = self.name.as_ref() {
            write!(formatter, "{}: ", name)?;
        }

        self.ty.fmt(formatter)?;
        write!(formatter, ",\n")
    }

    pub(crate) fn fmt_assoc_type_value(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let name = self.name.as_ref().expect("associated type must be named");
        write!(formatter, "type {} = ", name)?;
        self.ty.fmt(formatter)?;
        write!(formatter, ";\n")
    }

    impl_attr_methods!(attrs);
    impl_doc_methods!(docs);
    impl_ty_methods!(field => ty);
    impl_vis_methods!(field => vis);
}
