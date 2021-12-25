use std::fmt::{self, Write};

use crate::attributes::Attributes;
use crate::bounds::Bounds;
use crate::field::Field;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::generics::Generics;
use crate::type_def::Type;

use crate::impl_macros::{
    impl_attr_methods,
    impl_bounds_methods,
    impl_generic_methods,
};

/// Defines an impl block.
#[derive(Debug, Clone)]
pub struct Impl {
    /// The struct being implemented
    target: Type,

    /// Impl level generics
    generics: Generics,

    /// If implementing a trait
    impl_trait: Option<Type>,

    /// Associated types
    assoc_tys: Vec<Field>,

    /// Bounds
    bounds: Bounds,

    fns: Vec<Function>,

    attrs: Attributes,
}

impl Impl {
    /// Return a new impl definition
    pub fn new<T>(target: T) -> Self
    where
        T: Into<Type>,
    {
        Impl {
            target: target.into(),
            generics: Generics::default(),
            impl_trait: None,
            assoc_tys: vec![],
            bounds: Bounds::default(),
            fns: vec![],
            attrs: Attributes::default(),
        }
    }

    /// Add a generic to the target type.
    pub fn target_generic<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.target.push_generic(ty);
        self
    }

    /// Set the trait that the impl block is implementing.
    pub fn impl_trait<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.impl_trait = Some(ty.into());
        self
    }

    /// Set an associated type.
    pub fn associate_type<S, T>(&mut self, name: S, ty: T) -> &mut Self
    where
        S: AsRef<str>,
        T: Into<Type>,
    {
        self.assoc_tys.push(Field::new_named(name, ty));
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: &str) -> &mut Function {
        self.push_fn(Function::new(name));
        self.fns.last_mut().unwrap()
    }

    /// Push a function definition.
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.fns.push(item);
        self
    }

    /// Formats the impl block using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.attrs.fmt_attrs(fmt)?;

        write!(fmt, "impl")?;
        self.generics.fmt_generics(fmt)?;

        if let Some(ref t) = self.impl_trait {
            write!(fmt, " ")?;
            t.fmt(fmt)?;
            write!(fmt, " for")?;
        }

        write!(fmt, " ")?;
        self.target.fmt(fmt)?;

        self.bounds.fmt_bounds(fmt)?;

        fmt.block(|fmt| {
            // format associated types
            if !self.assoc_tys.is_empty() {
                for ty in &self.assoc_tys {
                    ty.fmt_assoc_type_value(fmt)?;
                }
            }

            for (i, func) in self.fns.iter().enumerate() {
                if i != 0 || !self.assoc_tys.is_empty() {
                    write!(fmt, "\n")?;
                }

                func.fmt(false, fmt)?;
            }

            Ok(())
        })
    }

    impl_attr_methods!(attrs);
    impl_bounds_methods!(bounds);
    impl_generic_methods!(generics);
}
