use std::fmt::{self, Write};

use crate::associated_type::AssociatedType;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::type_def::{Type, TypeDef, impl_type_def_passthrough};

/// Define a trait.
#[derive(Debug, Clone)]
pub struct Trait {
    type_def: TypeDef,
    parents: Vec<Type>,
    associated_tys: Vec<AssociatedType>,
    fns: Vec<Function>,
}

impl Trait {
    /// Return a trait definition with the provided name
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>
    {
        Trait {
            type_def: TypeDef::new(name),
            parents: vec![],
            associated_tys: vec![],
            fns: vec![],
        }
    }

    /// Add a parent trait.
    pub fn parent<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.parents.push(ty.into());
        self
    }


    /// Add an associated type. Returns a mutable reference to the new
    /// associated type for futher configuration.
    pub fn associated_type(&mut self, name: &str) -> &mut AssociatedType {
        self.associated_tys.push(AssociatedType::new(name));
        self.associated_tys.last_mut().unwrap()
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: &str) -> &mut Function {
        self.push_fn(Function::new_trait_fn(name));
        self.fns.last_mut().unwrap()
    }

    /// Push a function definition.
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.fns.push(item);
        self
    }

    /// Formats the scope using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("trait", &self.parents, fmt)?;

        fmt.block(|fmt| {
            let assoc = &self.associated_tys;

            // format associated types
            if !assoc.is_empty() {
                for assoc_ty in assoc {
                    assoc_ty.fmt_assoc_type(fmt)?;
                }
            }

            for (i, func) in self.fns.iter().enumerate() {
                if i != 0 || !assoc.is_empty() {
                    write!(fmt, "\n")?;
                }

                func.fmt(true, fmt)?;
            }

            Ok(())
        })
    }

    impl_type_def_passthrough!(type_def);
}
