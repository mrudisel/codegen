use std::fmt::{self, Write};
use std::iter::Extend;

use crate::formatter::Formatter;

/// Helper to implement both normal docs (prefixed with '///'), and module-level docs
/// (prefixed with '//!')
macro_rules! impl_docs {
    ($($(#[$attrs:meta])? ($name:ident, $prefix:literal)),* $(,)?) => {
        $(
            $(#[$attrs])?
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name {
                /// Inner documentation container. Each element should be 1 line of documentation
                lines: Vec<String>,
            }

            impl $name {
                /// Creates an empty documentation container
                pub fn empty() -> Self {
                    Self { lines: vec![] }
                }

                /// Creates a new documentation container with some initial text
                pub fn new<S>(doc: S) -> Self
                where
                    S: AsRef<str>
                {
                    let mut new = Self::empty();
                    new.push_doc(doc);
                    new
                }

                /// Creates a new documentation container with an initial set of text.
                pub fn new_with_lines<I, S>(iter: I) -> Self
                where
                    I: IntoIterator<Item = S>,
                    S: AsRef<str>
                {
                    let mut new = Self::empty();
                    new.push_docs(iter);
                    new
                }

                /// Clears the underlying container, but does not deallocate. Calls [`Vec::clear`]
                /// under the hood.
                pub fn clear_docs(&mut self) -> &mut Self {
                    self.lines.clear();
                    self
                }

                /// Returns the number of lines in the underlying documentation container.
                pub fn doc_line_count(&self) -> usize {
                    self.lines.len()
                }

                /// Checks if there is any documentation in the underlying container.
                pub fn is_docs_empty(&self) -> bool {
                    self.doc_line_count() == 0
                }

                /// Pushes a single item of documentation to the underlying container. Splits on
                /// '\n' to keep the underlying docs delimited by lines.
                pub fn push_doc<S>(&mut self, line: S) -> &mut Self
                where
                    S: AsRef<str>
                {
                    self.lines.extend(line.as_ref().lines().map(ToOwned::to_owned));
                    self
                }

                /// Pushes multiple items to the underlying documentation container. Helper
                /// function around [`push_doc`].
                ///
                /// [`push_doc`]: [`Self::push_doc`]
                pub fn push_docs<I, S>(&mut self, lines: I) -> &mut Self
                where
                    I: IntoIterator<Item = S>,
                    S: AsRef<str>
                {
                    for line in lines.into_iter() {
                        self.push_doc(line);
                    }

                    self
                }

                /// Overwrites any existing documentation with the given argument.
                pub fn set_doc<S>(&mut self, doc: S) -> &mut Self
                where
                    S: AsRef<str>
                {
                    self.clear_docs();
                    self.push_doc(doc);
                    self
                }

                /// Helper function for [`set_doc`], similar to the [`push_doc`]/[`push_docs`]
                /// functions for pushing rather than overwriting.
                ///
                /// [`set_doc`]: [`Self::set_doc`]
                /// [`push_doc`]: [`Self::push_doc`]
                /// [`push_docs`]: [`Self::push_docs`]
                pub fn set_docs<I, S>(&mut self, lines: I) -> &mut Self
                where
                    I: IntoIterator<Item = S>,
                    S: AsRef<str>
                {
                    // Clears the docs, but doenst de-allocate the underlying memory. That way we
                    // don't need to reallocate memory that we're immediately filling back up.
                    self.clear_docs();
                    self.push_docs(lines);
                    self
                }

                /// Formats the documentation, line by line.
                pub fn fmt_docs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
                    for line in self.lines.iter() {
                        if line.starts_with($prefix) {
                            write!(fmt, "{}", line)?;
                        }
                        else {
                            write!(fmt, $prefix, line)?;
                        }
                    }

                    Ok(())
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::empty()
                }
            }

            impl<S> Extend<S> for $name
            where
                S: AsRef<str>,
            {
                fn extend<I>(&mut self, iter: I)
                where
                    I: IntoIterator<Item = S>
                {
                    self.push_docs(iter);
                }
            }
        )*
    };
}

impl_docs! {
    /// Documentation container for any non-module item.
    (Docs, "/// {}\n"),
    /// Module level documentation container.
    (ModuleDocs, "//! {}\n"),
}



/// Helper macro to implement attribute methods that are both consistent across types, but that
/// also keep the builder pattern consistent.
///
/// There are two patterns that are matched, both prefixed differently.
///
/// See the documentation on [`impl_attr_methods!`] for usage, since that macro follows the same
/// logic + has the same prefixes.
///
/// [`impl_attr_methods!`]: <macro.impl_attr_methods.html>
macro_rules! impl_doc_methods {
    ($($inner:tt)+) => {
        /// Clears the underlying container, but does not deallocate. Calls [`Vec::clear`] under
        /// the hood.
        pub fn clear_docs(&mut self) -> &mut Self {
            self.$($inner)+.clear_docs();
            self
        }

        /// Returns the number of lines in the underlying documentation container.
        pub fn doc_line_count(&self) -> usize {
            self.$($inner)+.doc_line_count()
        }

        /// Checks if there is any documentation in the underlying container.
        pub fn is_docs_empty(&self) -> bool {
            self.$($inner)+.is_docs_empty()
        }

        /// Pushes a single item of documentation to the underlying container. Splits on '\n' to
        /// keep the underlying docs delimited by lines.
        pub fn push_doc<S>(&mut self, line: S) -> &mut Self
        where
            S: AsRef<str>
        {
            self.$($inner)+.push_doc(line);
            self
        }

        /// Pushes multiple items to the underlying documentation container. Helper function
        /// around [`push_doc`].
        ///
        /// [`push_doc`]: [`Self::push_doc`]
        pub fn push_docs<I, S>(&mut self, lines: I) -> &mut Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>
        {
            self.$($inner)+.push_docs(lines);
            self
        }

        /// Overwrites any existing documentation with the given argument.
        pub fn set_doc<S>(&mut self, doc: S) -> &mut Self
        where
            S: AsRef<str>
        {
            self.$($inner)+.set_doc(doc);
            self
        }

        /// Helper function for [`set_doc`], similar to the [`push_doc`]/[`push_docs`]
        /// functions for pushing rather than overwriting.
        ///
        /// [`set_doc`]: [`Self::set_doc`]
        /// [`push_doc`]: [`Self::push_doc`]
        /// [`push_docs`]: [`Self::push_docs`]
        pub fn set_docs<I, S>(&mut self, lines: I) -> &mut Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>
        {
            self.$($inner)+.set_docs(lines);
            self
        }
    };
}

pub(crate) use impl_doc_methods;
