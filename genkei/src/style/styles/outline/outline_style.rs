use crate::{Outline, Style, StyleError};
use std::fmt::Write;

/// Represents the outline-style style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OutlineStyle {
    /// outline-style: solid
    Solid,
    /// outline-style: dashed
    Dashed,
    /// outline-style: dotted
    Dotted,
    /// outline-style: double
    Double,
    /// outline-style: groove
    Groove,
    /// outline-style: ridge
    Ridge,
    /// outline-style: inset
    Inset,
    /// outline-style: outset
    Outset,
    /// outline-style: hidden
    Hidden,
    /// outline-style: none
    None,
}

impl From<OutlineStyle> for Style {
    fn from(value: OutlineStyle) -> Self {
        Outline::Style(value).into()
    }
}

impl OutlineStyle {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            OutlineStyle::Solid => write!(stream, "os-solid")?,
            OutlineStyle::Dashed => write!(stream, "os-dashed")?,
            OutlineStyle::Dotted => write!(stream, "os-dotted")?,
            OutlineStyle::Double => write!(stream, "os-double")?,
            OutlineStyle::Groove => write!(stream, "os-groove")?,
            OutlineStyle::Ridge => write!(stream, "os-ridge")?,
            OutlineStyle::Inset => write!(stream, "os-inset")?,
            OutlineStyle::Outset => write!(stream, "os-outset")?,
            OutlineStyle::Hidden => write!(stream, "os-hidden")?,
            OutlineStyle::None => write!(stream, "os-none")?,
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
            OutlineStyle::Solid => write!(stream, "outline-style:solid")?,
            OutlineStyle::Dashed => write!(stream, "outline-style:dashed")?,
            OutlineStyle::Dotted => write!(stream, "outline-style:dotted")?,
            OutlineStyle::Double => write!(stream, "outline-style:double")?,
            OutlineStyle::Groove => write!(stream, "outline-style:groove")?,
            OutlineStyle::Ridge => write!(stream, "outline-style:ridge")?,
            OutlineStyle::Inset => write!(stream, "outline-style:inset")?,
            OutlineStyle::Outset => write!(stream, "outline-style:outset")?,
            OutlineStyle::Hidden => write!(stream, "outline-style:hidden")?,
            OutlineStyle::None => write!(stream, "outline-style:none")?,
        };

        Ok(())
    }
}
