use crate::{Justify, Style, StyleError};
use std::fmt::Write;

/// Represents the justify-self style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JustifySelf {
    /// justify-self: auto
    Auto,
    /// justify-self: start
    Start,
    /// justify-self: end
    End,
    /// justify-self: center
    Center,
    /// justify-self: stretch
    Stretch,
}

impl From<JustifySelf> for Style {
    fn from(value: JustifySelf) -> Self {
        Justify::Self_(value).into()
    }
}

impl JustifySelf {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            JustifySelf::Auto => write!(stream, "justify-self-auto")?,
            JustifySelf::Start => write!(stream, "justify-self-start")?,
            JustifySelf::End => write!(stream, "justify-self-end")?,
            JustifySelf::Center => write!(stream, "justify-self-center")?,
            JustifySelf::Stretch => write!(stream, "justify-self-stretch")?,
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
            JustifySelf::Auto => write!(stream, "justify-self:auto")?,
            JustifySelf::Start => write!(stream, "justify-self:start")?,
            JustifySelf::End => write!(stream, "justify-self:end")?,
            JustifySelf::Center => write!(stream, "justify-self:center")?,
            JustifySelf::Stretch => write!(stream, "justify-self:stretch")?,
        };

        Ok(())
    }
}
