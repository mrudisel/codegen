use std::fmt::{self, Write};

use crate::formatter::Formatter;
use crate::type_def::Type;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generics {
    lifetimes: Vec<String>,
    generics: Vec<Type>,
}

impl Generics {
    pub fn new() -> Self {
        Self {
            lifetimes: vec![],
            generics: vec![],
        }
    }

    pub fn push_lifetime<S>(&mut self, lifetime: S) -> &mut Self
    where
        S: AsRef<str>
    {
        self.lifetimes.push(lifetime.as_ref().to_owned());
        self
    }

    pub fn extend_lifetimes<I, S>(&mut self, lifetimes: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>
    {
        self.lifetimes.extend(lifetimes.into_iter().map(|l| l.as_ref().to_owned()));
        self
    }

    pub fn clear_lifetimes(&mut self) -> &mut Self {
        self.lifetimes.clear();
        self
    }

    pub fn push_generic<T>(&mut self, generic: T) -> &mut Self
    where
        T: Into<Type>
    {
        self.generics.push(generic.into());
        self
    }

    pub fn extend_generics<I, T>(&mut self, generics: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Type>
    {
        self.generics.extend(generics.into_iter().map(|t| t.into()));
        self
    }

    pub fn clear_generics(&mut self) -> &mut Self {
        self.generics.clear();
        self
    }

    /// Format generics.
    pub(crate) fn fmt_generics(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if !self.lifetimes.is_empty() || !self.generics.is_empty() {
            write!(fmt, "<")?;

            for (idx, lifetime) in self.lifetimes.iter().enumerate() {
                write!(fmt, "{}", lifetime)?;

                if !self.generics.is_empty() || idx != self.lifetimes.len() - 1 {
                    write!(fmt, ", ")?;
                }
            }

            for (idx, generic) in self.generics.iter().enumerate() {
                generic.fmt(fmt)?;

                if idx != self.generics.len() - 1 {
                    write!(fmt, ", ")?;
                }
            }

            write!(fmt, ">")?;
        }

        Ok(())
    }
}



impl Default for Generics {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_generic_methods {
    ($($inner:tt)+) => {
        /// Pushes a lifetime to the container
        pub fn push_lifetime<S>(&mut self, lifetime: S) -> &mut Self
        where
            S: AsRef<str>
        {
            self.$($inner)+.push_lifetime(lifetime);
            self
        }

        /// Extends multiple lifetimes to the inner container
        pub fn extend_lifetimes<I, S>(&mut self, lifetimes: I) -> &mut Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>
        {
            self.$($inner)+.extend_lifetimes(lifetimes);
            self
        }

        /// Clears all lifetimes from the underlying container.
        pub fn clear_lifetimes(&mut self) -> &mut Self {
            self.$($inner)+.clear_lifetimes();
            self
        }

        /// Pushes a generic type to the inner container
        pub fn push_generic<T>(&mut self, generic: T) -> &mut Self
        where
            T: Into<$crate::type_def::Type>
        {
            self.$($inner)+.push_generic(generic);
            self
        }

        /// Extends multiple generic types to the inner container
        pub fn extend_generics<I, T>(&mut self, generics: I) -> &mut Self
        where
            I: IntoIterator<Item = T>,
            T: Into<$crate::type_def::Type>
        {
            self.$($inner)+.extend_generics(generics);
            self
        }

        /// Clears all generics from the inner container.
        pub fn clear_generics(&mut self) -> &mut Self {
            self.$($inner)+.clear_generics();
            self
        }
    };
}

pub(crate) use impl_generic_methods;
