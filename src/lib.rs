#![deny(missing_debug_implementations, missing_docs)]

//! Provides a builder API for generating Rust code.
//!
//! The general strategy for using the crate is as follows:
//!
//! 1. Create a `Scope` instance.
//! 2. Use the builder API to add elements to the scope.
//! 3. Call `Scope::to_string()` to get the generated code.
//!
//! For example:
//!
//! ```rust
//! use codegen::Scope;
//!
//! let mut scope = Scope::new();
//!
//! scope.new_struct("Foo")
//!     .derive("Debug")
//!     .field("one", "usize")
//!     .field("two", "String");
//!
//! println!("{}", scope.to_string());
//! ```

mod associated_type;
mod attributes;
mod block;
mod body;
mod bounds;
mod docs;
mod enum_gen;
mod field;
mod fields;
mod formatter;
mod function;
mod generics;
mod impl_gen;
mod import;
mod item;
mod module;
mod scope;
mod struct_gen;
mod trait_gen;
mod type_def;
mod variant;
mod vis;



pub use associated_type::*;
pub use attributes::*;
pub use block::*;
pub use bounds::Bound;
pub use enum_gen::*;
pub use field::*;
pub use formatter::*;
pub use function::*;
pub use impl_gen::*;
pub use import::*;
pub use module::*;
pub use scope::*;
pub use struct_gen::*;
pub use trait_gen::*;
pub use type_def::Type;
pub use variant::*;
pub use vis::*;



pub(crate) mod impl_macros {
    pub(crate) use crate::attributes::impl_attr_methods;
    pub(crate) use crate::bounds::{impl_bound_methods, impl_bounds_methods};
    pub(crate) use crate::docs::impl_doc_methods;
    pub(crate) use crate::generics::impl_generic_methods;
    pub(crate) use crate::type_def::impl_ty_methods;
    pub(crate) use crate::vis::impl_vis_methods;
}
