use std::fmt::{self, Write};

use crate::attributes::Attributes;
use crate::bounds::Bounds;
use crate::docs::Docs;
use crate::formatter::Formatter;
use crate::generics::Generics;
use crate::vis::Vis;


use crate::impl_macros::{
    impl_attr_methods,
    impl_bounds_methods,
    impl_doc_methods,
    impl_generic_methods,
    impl_vis_methods,
};



/// Defines a type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    name: String,
    docs: Docs,
    generics: Generics,
}

impl Type {
    /// Return a new type with the given name.
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>
    {
        Type {
            name: name.as_ref().to_owned(),
            docs: Docs::default(),
            generics: Generics::default(),
        }
    }

    /// Returns the name of this type.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Rewrite the `Type` with the provided path
    ///
    /// TODO: Is this needed?
    pub fn path(&self, path: &str) -> Type {
        // TODO: This isn't really correct
        assert!(!self.name.contains("::"));

        let mut name = path.to_string();
        name.push_str("::");
        name.push_str(&self.name);

        Type {
            name,
            docs: self.docs.clone(),
            generics: self.generics.clone(),
        }
    }

    /// Formats the struct using the given formatter.
    pub(crate) fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.name)?;
        self.generics.fmt_generics(fmt)
    }

    impl_generic_methods!(generics);
}


impl<S> From<S> for Type
where
    S: AsRef<str>
{
    fn from(src: S) -> Self {
        Self::new(src)
    }
}


impl<'a> From<&'a Type> for Type {
    fn from(src: &'a Type) -> Self {
        src.clone()
    }
}


macro_rules! impl_ty_methods {
    (field => $($inner:tt)+) => {
        /// Gets a reference to the type.
        pub fn ty(&self) -> &$crate::type_def::Type {
            &self.$($inner)+
        }

        /// Gets a mutable reference to the type.
        pub fn ty_mut(&mut self) -> &mut $crate::type_def::Type {
            &mut self.$($inner)+
        }
    };
    ($($inner:tt)+) => {
        /// Gets a reference to this type.
        pub fn ty(&self) -> &$crate::type_def::Type {
            self.$($inner)+.ty()
        }

        /// Gets a mutable reference to this type.
        pub fn ty_mut(&mut self) -> &mut $crate::type_def::Type {
            self.$($inner)+.ty_mut()
        }
    };
}

pub(crate) use impl_ty_methods;


/// Defines a type definition.
#[derive(Debug, Clone)]
pub struct TypeDef {
    ty: Type,
    vis: Vis,
    docs: Docs,
    derive: Vec<String>,
    allow: Vec<String>,
    repr: Option<String>,
    bounds: Bounds,
    /// Attributes other than derive/allow/repr/docs attrs.
    attrs: Attributes,
}

impl TypeDef {
    /// Return a structure definition with the provided name
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>
    {
        TypeDef {
            ty: Type::new(name),
            vis: Vis::Private,
            docs: Docs::default(),
            derive: vec![],
            allow: vec![],
            repr: None,
            bounds: Bounds::default(),
            attrs: Attributes::default(),
        }
    }

    pub fn derive<S>(&mut self, name: S)
    where
        S: AsRef<str>
    {
        self.derive.push(name.as_ref().to_owned());
    }

    pub fn derive_many<I, S>(&mut self, iter: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        iter.into_iter()
            .for_each(|inner| self.derive(inner.as_ref()));
    }

    pub fn allow<S>(&mut self, allow: S)
    where
        S: AsRef<str>
    {
        self.allow.push(allow.as_ref().to_owned());
    }

    pub fn repr<S>(&mut self, repr: S)
    where
        S: AsRef<str>
    {
        self.repr = Some(repr.as_ref().to_owned());
    }

    pub fn fmt_head<S, P>(&self, keyword: S, parents: P, fmt: &mut Formatter<'_>) -> fmt::Result
    where
        S: AsRef<str>,
        P: AsRef<[Type]>,
    {
        self.docs.fmt_docs(fmt)?;
        self.fmt_allow(fmt)?;
        self.fmt_derive(fmt)?;
        self.fmt_repr(fmt)?;
        self.attrs.fmt_attrs(fmt)?;

        self.vis.fmt(fmt)?;

        write!(fmt, "{} ", keyword.as_ref())?;
        self.ty.fmt(fmt)?;

        let parents = parents.as_ref();
        if !parents.is_empty() {
            for (i, ty) in parents.iter().enumerate() {
                if i == 0 {
                    write!(fmt, ": ")?;
                } else {
                    write!(fmt, " + ")?;
                }

                ty.fmt(fmt)?;
            }
        }

        self.bounds.fmt_bounds(fmt)?;

        Ok(())
    }

    fn fmt_allow(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for allow in &self.allow {
            write!(fmt, "#[allow({})]\n", allow)?;
        }

        Ok(())
    }

    fn fmt_repr(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref repr) = self.repr {
            write!(fmt, "#[repr({})]\n", repr)?;
        }

        Ok(())
    }

    fn fmt_derive(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if !self.derive.is_empty() {
            write!(fmt, "#[derive(")?;

            for (i, name) in self.derive.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                write!(fmt, "{}", name)?;
            }

            write!(fmt, ")]\n")?;
        }

        Ok(())
    }

    impl_attr_methods!(attrs);
    impl_bounds_methods!(bounds);
    impl_doc_methods!(docs);
    impl_generic_methods!(ty);
    impl_ty_methods!(field => ty);
    impl_vis_methods!(field => vis);
}




macro_rules! impl_type_def_passthrough {
    ($field:ident) => {
        // handle all the generic passthroughs
        $crate::impl_macros::impl_attr_methods!($field);
        $crate::impl_macros::impl_bounds_methods!($field);
        $crate::impl_macros::impl_doc_methods!($field);
        $crate::impl_macros::impl_generic_methods!($field);
        $crate::impl_macros::impl_ty_methods!($field);
        $crate::impl_macros::impl_vis_methods!($field);

        // then pass through the inner TypeDef functions:

        /// Add a single macro to derive
        pub fn derive<S>(&mut self, name: S) -> &mut Self
        where
            S: AsRef<str>
        {
            self.$field.derive(name);
            self
        }

        /// Add many macros to derive.
        pub fn derive_many<I, S>(&mut self, iter: I) -> &mut Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>,
        {
            self.$field.derive_many(iter);
            self
        }

        /// Add lint allow flags
        pub fn allow<S>(&mut self, allow: S) -> &mut Self
        where
            S: AsRef<str>
        {
            self.$field.allow(allow);
            self
        }

        /// Add repr flags
        pub fn repr<S>(&mut self, repr: S) -> &mut Self
        where
            S: AsRef<str>
        {
            self.$field.repr(repr);
            self
        }
    };
}

pub(crate) use impl_type_def_passthrough;
