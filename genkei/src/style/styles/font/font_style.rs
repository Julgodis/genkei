use crate::{Font, Style, StyleError};
use std::fmt::Write;

/// Font style attributes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontStyle {
    /// font-style: normal
    Normal,
    /// font-style: italic
    Italic,
}

impl From<FontStyle> for Style {
    fn from(value: FontStyle) -> Self {
        Style::Font(Font::Style(value))
    }
}

impl FontStyle {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            FontStyle::Normal => write!(stream, "fst-normal")?,
            FontStyle::Italic => write!(stream, "fst-italic")?,
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
            FontStyle::Normal => write!(stream, "font-style:normal")?,
            FontStyle::Italic => write!(stream, "font-style:italic")?,
        };

        Ok(())
    }
}
