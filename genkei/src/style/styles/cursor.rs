use crate::Style;
use crate::StyleError;
use crate::Styleable;
use std::fmt::Write;

/// Represents the cursor style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Cursor {
    /// cursor: auto
    Auto,
    /// cursor: default
    Default,
    /// cursor: pointer
    Pointer,
}

impl From<Cursor> for Style {
    fn from(value: Cursor) -> Self {
        Style::Cursor(value)
    }
}

impl Cursor {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Cursor::Auto => write!(stream, "cursor-auto")?,
            Cursor::Default => write!(stream, "cursor-default")?,
            Cursor::Pointer => write!(stream, "cursor-pointer")?,
        };

        Ok(())
    }

    pub(crate) fn write_css_statement<T>(
        &self,
        stream: &mut String,
        _options: &T,
    ) -> Result<(), StyleError>
    where
        T: crate::StyleOptions,
    {
        match self {
            Cursor::Auto => write!(stream, "cursor:auto")?,
            Cursor::Default => write!(stream, "cursor:default")?,
            Cursor::Pointer => write!(stream, "cursor:pointer")?,
        };

        Ok(())
    }
}

impl<T> CursorTrait for T where T: Styleable {}

/// A trait for the cursor style attributes.
pub trait CursorTrait: Styleable {
    #[inline]
    fn cursor(self, value: impl Into<Cursor>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn cursor_pointer(self) -> Self::Output {
        self.cursor(Cursor::Pointer)
    }
}
