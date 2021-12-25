use std::fmt::{self, Write};

use crate::formatter::Formatter;

use crate::type_def::Type;

/// Represents a collection of bounds for a single type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {
    name: String,
    bounds: Vec<Type>,
}

/// Formatting helper for chaining bounds in different scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundEndsWith {
    SemiColon,
    Comma,
}

impl Bound {
    /// Creates a new type bound with no trait requirements.
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>
    {
        Self {
            name: name.as_ref().to_owned(),
            bounds: vec![],
        }
    }

    /// Creates a new type bound with an iterator of trait requirements
    pub fn new_with_bounds<S, I, T>(name: S, bounds: I) -> Self
    where
        S: AsRef<str>,
        I: IntoIterator<Item = T>,
        T: Into<Type>,
    {
        Self {
            name: name.as_ref().to_owned(),
            bounds: bounds.into_iter().map(|b| b.into()).collect(),
        }
    }

    /// Creates a new type bound with a single trait requirements
    pub fn new_with_bound<S, T>(name: S, bound: T) -> Self
    where
        S: AsRef<str>,
        T: Into<Type>,
    {
        Self::new_with_bounds(name, vec![bound.into()])
    }

    /// Returns the name of the bound
    pub fn name(&self) -> &str {
        self.name.as_str()
    }


    /// Returns a slice of the bounding types
    pub fn bounds(&self) -> &[Type] {
        self.bounds.as_slice()
    }

    /// Returns the number of bounds
    pub fn bound_count(&self) -> usize {
        self.bounds.len()
    }

    /// Checks if this bound has type/trait requirements.
    pub fn has_inner_bounds(&self) -> bool {
        self.bounds.is_empty()
    }

    /// Clears all bound requirements
    pub fn clear_bounds(&mut self) -> &mut Self {
        self.bounds.clear();
        self
    }

    /// Pushes a single bound requirement
    pub fn push_bound<T>(&mut self, bound: T) -> &mut Self
    where
        T: Into<Type>
    {
        self.bounds.push(bound.into());
        self
    }

    /// Extends the inner container with more bounds
    pub fn extend_bounds<I, T>(&mut self, bounds: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Type>,
    {
        self.bounds.extend(bounds.into_iter().map(|b| b.into()));
        self
    }

    pub(crate) fn fmt_bound(
        &self,
        formatter: &mut Formatter<'_>,
        ends_with: BoundEndsWith,
    ) -> fmt::Result {
        if self.bounds.is_empty() {
            write!(formatter, "{}", self.name)?;
        }
        else {
            write!(formatter, "{}:", self.name)?;

            for (i, ty) in self.bounds.iter().enumerate() {
                if i != 0 {
                    write!(formatter, " + ")?
                }
                ty.fmt(formatter)?;
            }
        }

        match ends_with {
            BoundEndsWith::SemiColon => {
                write!(formatter, ";\n")?;
            },
            BoundEndsWith::Comma => {
                write!(formatter, ",\n")?;
            }
        }

        Ok(())
    }
}

impl<S, I, T> From<(S, I)> for Bound
where
    S: AsRef<str>,
    I: IntoIterator<Item = T>,
    T: Into<Type>,
{
    fn from(bound_tup: (S, I)) -> Self {
        Self::new_with_bounds(bound_tup.0, bound_tup.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bounds {
    bounds: Vec<Bound>,
}

impl Bounds {
    pub fn new() -> Self {
        Self { bounds: vec![] }
    }

    pub fn new_with_bound(bound: Bound) -> Self {
        Self { bounds: vec![bound] }
    }

    pub fn new_with_bounds<I>(bounds: I) -> Self
    where
        I: IntoIterator<Item = Bound>
    {
        Self { bounds: bounds.into_iter().collect() }
    }

    pub fn bound_count(&self) -> usize {
        self.bounds.len()
    }

    pub fn has_bounds(&self) -> bool {
        self.bounds.is_empty()
    }

    pub fn clear_bounds(&mut self) -> &mut Self {
        self.bounds.clear();
        self
    }

    pub fn push_bound(&mut self, bound: Bound) -> &mut Self {
        self.bounds.push(bound);
        self
    }

    pub fn extend_bounds<I>(&mut self, bounds: I) -> &mut Self
    where
        I: IntoIterator<Item = Bound>
    {
        self.bounds.extend(bounds);
        self
    }

    pub(crate) fn fmt_bounds(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if !self.bounds.is_empty() {
            write!(formatter, "\nwhere\n")?;

            for bound in self.bounds.iter() {
                bound.fmt_bound(formatter, BoundEndsWith::Comma)?;
            }
        }

        Ok(())
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Self::new()
    }
}


macro_rules! impl_bound_methods {
    ($($inner:tt)+) => {
        /// The number of trait this bound must satisfy
        pub fn bound_count(&self) -> usize {
            self.$($inner)+.bound_count()
        }

        /// Whether or not this has any bounds
        pub fn has_inner_bounds(&self) -> bool {
            self.$($inner)+.has_inner_bounds()
        }

        /// Clears the bounds in the inner container.
        pub fn clear_bounds(&mut self) -> &mut Self {
            self.$($inner)+.clear_bounds();
            self
        }

        /// Adds a trait bound that must be satified by the type.
        pub fn push_bound<T>(&mut self, bound: T) -> &mut Self
        where
            T: Into<Type>
        {
            self.$($inner)+.push_bound(bound);
            self
        }

        /// Extends the bounds with a iterable collection of traits.
        pub fn extend_bounds<I, T>(&mut self, bounds: I) -> &mut Self
        where
            I: IntoIterator<Item = T>,
            T: Into<Type>,
        {
            self.$($inner)+.extend_bounds(bounds);
            self
        }
    };
}

macro_rules! impl_bounds_methods {
    ($($inner:tt)+) => {
        /// Gets the number of bounds across all types
        pub fn bound_count(&self) -> usize {
            self.$($inner)+.bound_count()
        }

        /// Whether or not this has any bounds for any types
        pub fn has_bounds(&self) -> bool {
            self.$($inner)+.has_bounds()
        }

        /// Clears all bounds for all types
        pub fn clear_bounds(&mut self) -> &mut Self {
            self.$($inner)+.clear_bounds();
            self
        }

        /// Pushes a new bound for a given type.
        pub fn push_bound(&mut self, bound: $crate::bounds::Bound) -> &mut Self {
            self.$($inner)+.push_bound(bound);
            self
        }

        /// Extends the inner container with multiple bounds.
        pub fn extend_bounds<I>(&mut self, bounds: I) -> &mut Self
        where
            I: IntoIterator<Item = $crate::bounds::Bound>
        {
            self.$($inner)+.extend_bounds(bounds);
            self
        }
    };
}

pub(crate) use impl_bound_methods;
pub(crate) use impl_bounds_methods;
