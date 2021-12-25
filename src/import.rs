use crate::vis::Vis;

use crate::impl_macros::impl_vis_methods;

/// Defines an import (`use` statement).
#[derive(Debug, Clone)]
pub struct Import {
    line: String,

    /// Function visibility
    pub vis: Vis,
}

impl Import {
    /// Return a new import.
    pub fn new<S, T>(path: S, ty: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Import {
            line: format!("{}::{}", path.as_ref(), ty.as_ref()),
            vis: Vis::default(),
        }
    }

    impl_vis_methods!(field => vis);
}
