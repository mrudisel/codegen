use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// A container for attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attributes {
    /// The inner attribute container. Each element should be a single attribute.
    attrs: Vec<String>,
}

impl Attributes {
    /// Creates a new, empty attribute container.
    pub fn new() -> Self {
        Self { attrs: vec![] }
    }

    /// Creates a new attribute container with an initial attribute.
    pub fn new_with_attr<S>(attr: S) -> Self
    where
        S: AsRef<str>
    {
        let mut attrs = Self::new();
        attrs.push_attr(attr);
        attrs
    }

    /// Creates a new attribute container with an initial attribute.
    pub fn new_with_attrs<I, S>(iter: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>
    {
        let mut attrs = Self::new();
        attrs.extend_attrs(iter);
        attrs
    }

    /// Gets the number of attributes already in this container.
    pub fn attr_count(&self) -> usize {
        self.attrs.len()
    }

    /// Determines if this attributes container is empty or not.
    pub fn is_attrs_empty(&self) -> bool {
        self.attrs.is_empty()
    }

    /// Clears any attributes that are set on this item.
    pub fn clear_attrs(&mut self) -> &mut Self {
        self.attrs.clear();
        self
    }

    /// Returns the slice of attributes set on this item.
    pub fn attrs(&self) -> &[String] {
        self.attrs.as_slice()
    }

    /// Checks if an attribute is already set in the underlying container.
    pub fn has_attr<S>(&self, attr: S) -> bool
    where
        S: AsRef<str>
    {
        let attr = attr.as_ref();
        self.attrs.iter()
            .find(|existing| *existing == attr)
            .is_some()
    }

    /// Sets the attributes on this item. Overwrites any existing attributes. Use
    /// [`push_attr`] or [`push_attrs`] to add to an existing set of attributes.
    pub fn set_attrs<I, S>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.attrs = iter.into_iter()
            .map(|item| item.as_ref().to_owned())
            .collect();

        self
    }

    /// Pushes an attribute onto an existing set of attributes.
    pub fn push_attr<S>(&mut self, attr: S) -> &mut Self
    where
        S: AsRef<str>
    {
        self.attrs.push(attr.as_ref().to_owned());
        self
    }

    /// Takes an iterator of attributes, and extends them onto an existing set of attributes.
    pub fn extend_attrs<I, S>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.attrs.extend(iter.into_iter().map(|attr| attr.as_ref().to_owned()));
        self
    }

    /// Write out and format the attributes. If any of the inner attributes are missing the
    /// wrapping '#[...]' brackets, they'll be added here.
    pub(crate) fn fmt_attrs(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        for attr in self.attrs.iter() {
            if !attr.starts_with("#[") {
                write!(formatter, "#[")?;
            }

            write!(formatter, "{}", attr)?;

            if !attr.ends_with("]") {
                write!(formatter, "]")?;
            }

            write!(formatter, "\n")?;
        }

        Ok(())
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<S> for Attributes
where
    S: AsRef<str>
{
    fn from(attr: S) -> Self {
        Self::new_with_attr(attr)
    }
}


/// Helper macro to implement attribute methods on types that contain attributes.
/// Since this crate relies on the builder pattern, having methods to modify nested fields
/// and still return the parent type is desirable. This macro does that by defining functions
/// identical to the public facing [`Attributes`] methods, but that call the method on the inner
/// type, then return the parent type to keep the builder pattern consistent.
///
/// ## Example:
///
/// ```
/// # #[macro_use]
/// # extern crate codegen;
/// # use codegen::attributes::Attributes;
/// # fn main() {
/// // A generic item that has attributes.
/// struct Child {
///     pub attrs: Attributes,
/// }
///
/// impl Child {
///     // implements the same public facing methods, but that return the proper `Child` type.
///     impl_attr_methods!(attrs);
/// }
///
/// let mut child = Child { attrs: Attributes::new() };
///
/// child.push_attr("#[example_attr_a]");
///
/// // This also handles nesting, as long as the target is either an `Attributes` type, or a type
/// // that has used `impl_attr_methods!` to define those same methods.
/// struct Parent {
///     pub child: Child,
/// }
///
/// impl Parent {
///     impl_attr_methods!(child);
/// }
///
/// let mut parent = Parent { child };
///
/// parent.push_attr("#[example_attr_b]");
///
/// assert_eq!(parent.child.attrs, vec!["#[example_attr_a]", "#[example_attr_b]"]);
/// # }
/// ```
macro_rules! impl_attr_methods {
    ($($inner:tt)+) => {
        /// Gets the number of attributes already in this container.
        pub fn attr_count(&self) -> usize {
            self.$($inner)+.attr_count()
        }

        /// Determines if this attributes container is empty or not.
        pub fn is_attrs_empty(&self) -> bool {
            self.$($inner)+.is_attrs_empty()
        }

        /// Clears any attributes that are set on this item.
        pub fn clear_attrs(&mut self) -> &mut Self {
            self.$($inner)+.clear_attrs();
            self
        }

        /// Returns the slice of attributes set on this item.
        pub fn attrs(&self) -> &[String] {
            self.$($inner)+.attrs()
        }

        /// Checks if an attribute is already set in the underlying container.
        pub fn has_attr<S>(&self, attr: S) -> bool
        where
            S: AsRef<str>
        {
            self.$($inner)+.has_attr(attr)
        }

        /// Sets the attributes on this item. Overwrites any existing attributes. Use
        /// [`push_attr`] or [`extend_attr`] to add to an existing set of attributes.
        pub fn set_attrs<I, S>(&mut self, iter: I) -> &mut Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>,
        {
            self.$($inner)+.set_attrs(iter);
            self
        }

        /// Pushes an attribute onto an existing set of attributes.
        pub fn push_attr<S>(&mut self, attr: S) -> &mut Self
        where
            S: AsRef<str>
        {
            self.$($inner)+.push_attr(attr);
            self
        }

        /// Takes an iterator of attributes, and extends them onto an existing set of attributes.
        pub fn extend_attrs<I, S>(&mut self, iter: I) -> &mut Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>,
        {
            self.$($inner)+.extend_attrs(iter);
            self
        }
    };
}

pub(crate) use impl_attr_methods;
