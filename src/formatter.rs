use std::fmt::{self, Write};


const DEFAULT_INDENT: usize = 4;

/// Configures how a scope is formatted.
#[derive(Debug)]
pub struct Formatter<'a> {
    /// Write destination
    dst: &'a mut String,

    /// Number of spaces to start a new line with.
    spaces: usize,

    /// Number of spaces per indentiation
    indent: usize,
}

impl<'a> Formatter<'a> {
    /// Return a new formatter that writes to the given string.
    pub fn new(dst: &'a mut String) -> Self {
        Formatter {
            dst,
            spaces: 0,
            indent: DEFAULT_INDENT,
        }
    }

    /// Wrap the given function inside a block.
    pub fn block<F>(&mut self, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        if !self.is_start_of_line() {
            write!(self, " ")?;
        }

        write!(self, "{{\n")?;
        self.indent(f)?;
        write!(self, "}}\n")?;
        Ok(())
    }

    /// Call the given function with the indentation level incremented by one.
    pub fn indent<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.spaces += self.indent;
        let ret = f(self);
        self.spaces -= self.indent;
        ret
    }

    /// Check if current destination is the start of a new line.
    pub fn is_start_of_line(&self) -> bool {
        self.dst.is_empty() || self.dst.as_bytes().last() == Some(&b'\n')
    }

    fn push_spaces(&mut self) {
        for _ in 0..self.spaces {
            self.dst.push_str(" ");
        }
    }
}

impl<'a> fmt::Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut first = true;
        let mut should_indent = self.is_start_of_line();

        for line in s.lines() {
            if !first {
                self.dst.push_str("\n");
            }

            first = false;

            let do_indent = should_indent && !line.is_empty() && line.as_bytes()[0] != b'\n';

            if do_indent {
                self.push_spaces();
            }

            // If this loops again, then we just wrote a new line
            should_indent = true;

            self.dst.push_str(line);
        }

        if s.as_bytes().last() == Some(&b'\n') {
            self.dst.push_str("\n");
        }

        Ok(())
    }
}
