use std::fmt::{self, Write};

use crate::attributes::Attributes;
use crate::block::Block;
use crate::body::Body;
use crate::bounds::Bounds;
use crate::docs::Docs;
use crate::field::Field;
use crate::formatter::Formatter;
use crate::generics::Generics;
use crate::type_def::Type;
use crate::vis::Vis;

use crate::impl_macros::{
    impl_attr_methods,
    impl_bounds_methods,
    impl_doc_methods,
    impl_generic_methods,
    impl_vis_methods,
};


/// Defines a function.
#[derive(Debug, Clone)]
pub struct Function {
    /// Name of the function
    name: String,
    /// Function documentation
    docs: Docs,
    /// A lint attribute used to suppress a warning or error
    allow: Option<String>,
    /// Function visibility
    vis: Vis,
    /// Function generics
    generics: Generics,
    /// If the function takes `&self` or `&mut self`
    arg_self: Option<&'static str>,
    /// Function arguments
    args: Vec<Field>,
    /// Return type
    ret: Option<Type>,
    /// Where bounds
    bounds: Bounds,
    /// Body contents
    body: Option<Vec<Body>>,
    /// Function attributes, e.g., `#[no_mangle]`.
    attrs: Attributes,
    /// Function `extern` ABI
    extern_abi: Option<String>,
    /// Whether or not this function is `async` or not
    is_async: bool,
}

impl Function {
    /// Creates a new function definition for a trait.
    pub fn new_trait_fn<S>(name: S) -> Self
    where
        S: AsRef<str>
    {
        Self {
            name: name.as_ref().to_owned(),
            docs: Docs::default(),
            allow: None,
            vis: Vis::default(),
            generics: Generics::default(),
            arg_self: None,
            args: vec![],
            ret: None,
            bounds: Bounds::default(),
            body: None,
            attrs: Attributes::default(),
            extern_abi: None,
            is_async: false,
        }
    }

    /// Return a new function definition.
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>
    {
        Function {
            name: name.as_ref().to_owned(),
            docs: Docs::default(),
            allow: None,
            vis: Vis::default(),
            generics: Generics::default(),
            arg_self: None,
            args: vec![],
            ret: None,
            bounds: Bounds::default(),
            body: Some(vec![]),
            attrs: Attributes::default(),
            extern_abi: None,
            is_async: false,
        }
    }

    /// Specify lint attribute to supress a warning or error.
    pub fn allow(&mut self, allow: &str) -> &mut Self {
        self.allow = Some(allow.to_string());
        self
    }

    /// Set whether this function is async or not
    pub fn set_async(&mut self, is_async: bool) -> &mut Self {
        self.is_async = is_async;
        self
    }

    /// Add `self` as a function argument.
    pub fn arg_self(&mut self) -> &mut Self {
        self.arg_self = Some("self");
        self
    }

    /// Add `&self` as a function argument.
    pub fn arg_ref_self(&mut self) -> &mut Self {
        self.arg_self = Some("&self");
        self
    }

    /// Add `&mut self` as a function argument.
    pub fn arg_mut_self(&mut self) -> &mut Self {
        self.arg_self = Some("&mut self");
        self
    }

    /// Add a function argument.
    pub fn arg<S, T>(&mut self, name: S, ty: T) -> &mut Self
    where
        S: AsRef<str>,
        T: Into<Type>,
    {
        self.args.push(Field::new_named(name, ty));
        self
    }

    /// Set the function return type.
    pub fn ret<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.ret = Some(ty.into());
        self
    }

    /// Push a line to the function implementation.
    pub fn line<T>(&mut self, line: T) -> &mut Self
    where
        T: ToString,
    {
        self.body
            .get_or_insert(vec![])
            .push(Body::String(line.to_string()));

        self
    }


    /// Specify an `extern` ABI for the function.
    /// ```
    /// use codegen::Function;
    ///
    /// let mut extern_func = Function::new("extern_func");
    ///
    /// // use the "C" calling convention
    /// extern_func.extern_abi("C");
    /// ```
    pub fn extern_abi(&mut self, abi: &str) -> &mut Self {
        self.extern_abi.replace(abi.to_string());
        self
    }

    /// Push a block to the function implementation
    pub fn push_block(&mut self, block: Block) -> &mut Self {
        self.body.get_or_insert(vec![]).push(Body::Block(block));

        self
    }

    /// Formats the function using the given formatter.
    pub fn fmt(&self, is_trait: bool, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.docs.fmt_docs(fmt)?;

        if let Some(ref allow) = self.allow {
            write!(fmt, "#[allow({})]\n", allow)?;
        }

        self.attrs.fmt_attrs(fmt)?;

        if is_trait {
            assert!(self.vis == Vis::Private, "trait fns do not have visibility modifiers");
        }

        self.vis.fmt(fmt)?;

        if let Some(ref extern_abi) = self.extern_abi {
            write!(fmt, "extern \"{extern_abi}\" ", extern_abi = extern_abi)?;
        }

        if self.is_async {
            write!(fmt, "async ")?;
        }

        write!(fmt, "fn {}", self.name)?;
        self.generics.fmt_generics(fmt)?;

        write!(fmt, "(")?;

        if let Some(ref s) = self.arg_self {
            write!(fmt, "{}", s)?;
        }

        for (i, arg) in self.args.iter().enumerate() {
            if i != 0 || self.arg_self.is_some() {
                write!(fmt, ", ")?;
            }

            arg.fmt_field(fmt)?;
        }

        write!(fmt, ")")?;

        if let Some(ref ret) = self.ret {
            write!(fmt, " -> ")?;
            ret.fmt(fmt)?;
        }

        self.bounds.fmt_bounds(fmt)?;

        match self.body {
            Some(ref body) => fmt.block(|fmt| {
                for b in body {
                    b.fmt(fmt)?;
                }

                Ok(())
            }),
            None => {
                if !is_trait {
                    panic!("impl blocks must define fn bodies");
                }

                write!(fmt, ";\n")
            }
        }
    }

    impl_attr_methods!(attrs);
    impl_bounds_methods!(bounds);
    impl_doc_methods!(docs);
    impl_generic_methods!(generics);
    impl_vis_methods!(field => vis);
}
