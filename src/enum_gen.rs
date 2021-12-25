use std::fmt;
use std::ops::{Deref, DerefMut};

use crate::formatter::Formatter;
use crate::type_def::{TypeDef, impl_type_def_passthrough};
use crate::variant::Variant;


/// Defines an enumeration.
#[derive(Debug, Clone)]
pub struct Enum {
    type_def: TypeDef,
    variants: Vec<Variant>,
}

impl Enum {
    /// Return a enum definition with the provided name.
    pub fn new(name: &str) -> Self {
        Enum {
            type_def: TypeDef::new(name),
            variants: vec![],
        }
    }

    /// Push a variant to the enum, returning a mutable reference to it.
    pub fn new_variant(&mut self, name: &str) -> &mut Variant {
        self.push_variant(Variant::new(name));
        self.variants.last_mut().unwrap()
    }

    /// Push a variant to the enum.
    pub fn push_variant(&mut self, item: Variant) -> &mut Self {
        self.variants.push(item);
        self
    }

    /// Formats the enum using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("enum", &[], fmt)?;

        fmt.block(|fmt| {
            for variant in &self.variants {
                variant.fmt(fmt)?;
            }

            Ok(())
        })
    }

    impl_type_def_passthrough!(type_def);
}

impl Deref for Enum {
    type Target = TypeDef;

    fn deref(&self) -> &Self::Target {
        &self.type_def
    }
}

impl DerefMut for Enum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.type_def
    }
}
