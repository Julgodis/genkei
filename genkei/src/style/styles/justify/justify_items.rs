use crate::{Justify, Style, StyleError};
use std::fmt::Write;

/// Represents the justify-items style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JustifyItems {
    /// justify-items: start
    Start,
    /// justify-items: end
    End,
    /// justify-items: center
    Center,
    /// justify-items: stretch
    Stretch,
}

impl From<JustifyItems> for Style {
    fn from(value: JustifyItems) -> Self {
        Justify::Items(value).into()
    }
}

impl JustifyItems {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            JustifyItems::Start => write!(stream, "justify-items-start")?,
            JustifyItems::End => write!(stream, "justify-items-end")?,
            JustifyItems::Center => write!(stream, "justify-items-center")?,
            JustifyItems::Stretch => write!(stream, "justify-items-stretch")?,
        }

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
            JustifyItems::Start => write!(stream, "justify-items:start")?,
            JustifyItems::End => write!(stream, "justify-items:end")?,
            JustifyItems::Center => write!(stream, "justify-items:center")?,
            JustifyItems::Stretch => write!(stream, "justify-items:stretch")?,
        };

        Ok(())
    }
}
