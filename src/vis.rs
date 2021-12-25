use std::fmt::{self, Write};

use crate::formatter::Formatter;


/// Visibility levels. A variant that can handle specific paths may be added in the future.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Vis {
    /// Private visibility. The default. Equivilent to having no visibility modifier.
    Private,
    /// Fully public
    Pub,
    /// Public to the entire crate.
    PubCrate,
    /// Public to only the parent module
    PubSuper,
}

impl Default for Vis {
    fn default() -> Self {
        Self::Private
    }
}

impl Vis {
    /// returns the raw, unformatted visibility modifier.
    pub fn vis_string(&self) -> Option<&'static str> {
        match self {
            Self::Private => None,
            Self::Pub => Some("pub"),
            Self::PubCrate => Some("pub(crate)"),
            Self::PubSuper => Some("pub(super)"),
        }
    }

    /// Formats the visibility on a given item.
    pub fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if let Some(vis_string) = self.vis_string() {
            write!(formatter, "{} ", vis_string)?;
        }

        Ok(())
    }
}

macro_rules! impl_vis_methods {
    (field => $($inner:tt)+) => {
        /// Sets the visibility of this item.
        pub fn set_vis(&mut self, vis: $crate::vis::Vis) -> &mut Self {
            self.$($inner)+ = vis;
            self
        }

        /// Get the visibility of this item
        pub fn get_vis(&self) -> $crate::vis::Vis {
            self.$($inner)+
        }

        /// Checks if this is a private item or not.
        pub fn is_private(&self) -> bool {
            matches!(self.get_vis(), $crate::vis::Vis::Private)
        }
    };
    ($($inner:tt)+) => {
        /// Sets the visibility of this item.
        pub fn set_vis(&mut self, vis: $crate::vis::Vis) -> &mut Self {
            self.$($inner)+.set_vis(vis);
            self
        }

        /// Get the visibility of this item
        pub fn get_vis(&self) -> $crate::vis::Vis {
            self.$($inner)+.get_vis()
        }

        /// Checks if this is a private item or not.
        pub fn is_private(&self) -> bool {
            matches!(self.get_vis(), $crate::vis::Vis::Private)
        }
    };
}

pub(crate) use impl_vis_methods;
