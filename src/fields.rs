use std::fmt::{self, Write};

use crate::attributes::Attributes;
use crate::field::Field;
use crate::formatter::Formatter;
use crate::type_def::Type;


/// Defines a set of fields.
#[derive(Debug, Clone)]
pub enum Fields {
    /// Represents an empty set of fields, i.e, a ZST struct.
    Empty,
    /// Represents a tuple of fields.
    Tuple(Vec<Type>),
    /// Represents many named fields, i.e like in a struct.
    Named(Vec<Field>),
}

impl Fields {
    pub fn add_named<S, T>(&mut self, name: S, ty: T) -> &mut Field
    where
        S: AsRef<str>,
        T: Into<Type>,
    {
        // Create + add the field
        self.named(name, ty);

        match self {
            Self::Named(named_fields) => named_fields.last_mut().unwrap(),
            _ => unreachable!("Fields::named did not catch an invalid Fields enum variant")
        }
    }

    pub fn add_tuple<T>(&mut self, r#type: T) -> &mut Self
    where
        T: Into<Type>
    {
        match self {
            Self::Tuple(tuple_fields) => tuple_fields.push(r#type.into()),
            _ => panic!("cannot call 'add_tuple' on a collection of non-tuple fields.")
        }

        self
    }

    pub fn push_named(&mut self, field: Field) -> &mut Self {
        match *self {
            Fields::Empty => {
                *self = Fields::Named(vec![field]);
            }
            Fields::Named(ref mut fields) => {
                fields.push(field);
            }
            _ => panic!("field list is named"),
        }

        self
    }

    pub fn named<S, T>(&mut self, name: S, ty: T) -> &mut Self
    where
        S: AsRef<str>,
        T: Into<Type>,
    {
        self.push_named(Field::new_named(name, ty));
        self
    }

    pub fn tuple<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        match *self {
            Fields::Empty => {
                *self = Fields::Tuple(vec![ty.into()]);
            }
            Fields::Tuple(ref mut fields) => {
                fields.push(ty.into());
            }
            _ => panic!("field list is tuple"),
        }

        self
    }

    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Fields::Named(ref fields) => {
                assert!(!fields.is_empty());

                fmt.block(|fmt| {
                    for field in fields {
                        field.fmt_field(fmt)?;
                    }

                    Ok(())
                })?;
            }
            Fields::Tuple(ref tys) => {
                assert!(!tys.is_empty());

                write!(fmt, "(")?;

                for (i, ty) in tys.iter().enumerate() {
                    if i != 0 {
                        write!(fmt, ", ")?;
                    }
                    ty.fmt(fmt)?;
                }

                write!(fmt, ")")?;
            }
            Fields::Empty => {}
        }

        Ok(())
    }
}
