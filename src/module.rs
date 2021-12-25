use std::fmt::{self, Write};

use crate::attributes::Attributes;
use crate::docs::ModuleDocs;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::scope::Scope;

use crate::vis::Vis;

use crate::enum_gen::Enum;
use crate::impl_gen::Impl;
use crate::struct_gen::Struct;
use crate::trait_gen::Trait;


use crate::impl_macros::{
    impl_attr_methods,
    impl_doc_methods,
    impl_vis_methods,
};


/// Defines a module.
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name
    name: String,

    /// Visibility
    vis: Vis,

    /// Module documentation
    docs: ModuleDocs,

    /// Contents of the module
    scope: Scope,

    /// attributes of the module. Useful for '#[cfg(test)]', etc.
    attrs: Attributes,
}

impl Module {
    /// Return a new, blank module
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>
    {
        Module {
            name: name.as_ref().to_owned(),
            vis: Vis::default(),
            docs: ModuleDocs::default(),
            scope: Scope::new(),
            attrs: Attributes::default(),
        }
    }

    /// Returns a mutable reference to the module's scope.
    pub fn scope(&mut self) -> &mut Scope {
        &mut self.scope
    }

    /// Returns the name of the module.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Import a type into the module's scope.
    ///
    /// This results in a new `use` statement bein added to the beginning of the
    /// module.
    pub fn import(&mut self, path: &str, ty: &str) -> &mut Self {
        self.scope.import(path, ty);
        self
    }

    /// Push a new module definition, returning a mutable reference to it.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it
    /// will return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn new_module<S>(&mut self, name: S) -> &mut Module
    where
        S: AsRef<str>,
    {
        self.scope.new_module(name)
    }

    /// Returns a reference to a module if it is exists in this scope.
    pub fn get_module<S>(&self, name: S) -> Option<&Module>
    where
        S: AsRef<str>,
    {
        self.scope.get_module(name)
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module_mut<S>(&mut self, name: S) -> Option<&mut Module>
    where
        S: AsRef<str>,
    {
        self.scope.get_module_mut(name)
    }

    /// Returns a mutable reference to a module, creating it if it does
    /// not exist.
    pub fn get_or_new_module(&mut self, name: &str) -> &mut Module {
        self.scope.get_or_new_module(name)
    }

    /// Push a module definition.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it will
    /// return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn push_module(&mut self, item: Module) -> &mut Self {
        self.scope.push_module(item);
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_struct(&mut self, name: &str) -> &mut Struct {
        self.scope.new_struct(name)
    }

    /// Push a structure definition
    pub fn push_struct(&mut self, item: Struct) -> &mut Self {
        self.scope.push_struct(item);
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: &str) -> &mut Function {
        self.scope.new_fn(name)
    }

    /// Push a function definition
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.scope.push_fn(item);
        self
    }

    /// Push a new enum definition, returning a mutable reference to it.
    pub fn new_enum(&mut self, name: &str) -> &mut Enum {
        self.scope.new_enum(name)
    }

    /// Push an enum definition
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.scope.push_enum(item);
        self
    }

    /// Push a new `impl` block, returning a mutable reference to it.
    pub fn new_impl(&mut self, target: &str) -> &mut Impl {
        self.scope.new_impl(target)
    }

    /// Push an `impl` block.
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.scope.push_impl(item);
        self
    }

    /// Push a trait definition
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.scope.push_trait(item);
        self
    }

    /// Formats the module using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.attrs.fmt_attrs(fmt)?;
        self.vis.fmt(fmt)?;
        write!(fmt, "mod {}", self.name)?;
        fmt.block(|fmt| self.scope.fmt(fmt))
    }

    impl_attr_methods!(attrs);
    impl_doc_methods!(docs);
    impl_vis_methods!(field => vis);
}
