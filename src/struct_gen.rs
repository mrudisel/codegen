use std::fmt::{self, Write};

use crate::field::Field;
use crate::fields::Fields;
use crate::formatter::Formatter;
use crate::type_def::{Type, TypeDef, impl_type_def_passthrough};


/// Defines a struct.
#[derive(Debug, Clone)]
pub struct Struct {
    type_def: TypeDef,

    /// Struct fields
    fields: Fields,
}

impl Struct {
    /// Return a structure definition with the provided name
    pub fn new(name: &str) -> Self {
        Struct {
            type_def: TypeDef::new(name),
            fields: Fields::Empty,
        }
    }

    /// Push a named field to the struct.
    ///
    /// A struct can either set named fields with this function or tuple fields
    /// with `push_tuple_field`, but not both.
    pub fn push_field(&mut self, field: Field) -> &mut Self {
        self.fields.push_named(field);
        self
    }

    /// Add a named field to the struct.
    ///
    /// A struct can either set named fields with this function or tuple fields
    /// with `tuple_field`, but not both.
    pub fn field<T>(&mut self, name: &str, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the struct.
    ///
    /// A struct can either set tuple fields with this function or named fields
    /// with `field`, but not both.
    pub fn tuple_field<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.tuple(ty);
        self
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("struct", &[], fmt)?;
        self.fields.fmt(fmt)?;

        match self.fields {
            Fields::Empty => {
                write!(fmt, ";\n")?;
            }
            Fields::Tuple(..) => {
                write!(fmt, ";\n")?;
            }
            _ => {}
        }

        Ok(())
    }

    // Implement all the methods that TypeDef provides, but made to conform to the builder pattern 
    impl_type_def_passthrough!(type_def);
}
