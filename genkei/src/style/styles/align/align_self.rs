use crate::{Align, Style, StyleError};
use std::fmt::Write;

/// Represents the align-self style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AlignSelf {
    /// align-self: auto
    Auto,
    /// align-self: flex-start
    FlexStart,
    /// align-self: flex-end
    FlexEnd,
    /// align-self: center
    Center,
    /// align-self: baseline
    Baseline,
    /// align-self: stretch
    Stretch,
}

impl From<AlignSelf> for Style {
    fn from(value: AlignSelf) -> Self {
        Align::Self_(value).into()
    }
}

impl AlignSelf {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            AlignSelf::Auto => write!(stream, "self-auto")?,
            AlignSelf::FlexStart => write!(stream, "self-start")?,
            AlignSelf::FlexEnd => write!(stream, "self-end")?,
            AlignSelf::Center => write!(stream, "self-center")?,
            AlignSelf::Baseline => write!(stream, "self-baseline")?,
            AlignSelf::Stretch => write!(stream, "self-stretch")?,
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
            AlignSelf::Auto => write!(stream, "align-self:auto")?,
            AlignSelf::FlexStart => write!(stream, "align-self:flex-start")?,
            AlignSelf::FlexEnd => write!(stream, "align-self:flex-end")?,
            AlignSelf::Center => write!(stream, "align-self:center")?,
            AlignSelf::Baseline => write!(stream, "align-self:baseline")?,
            AlignSelf::Stretch => write!(stream, "align-self:stretch")?,
        };

        Ok(())
    }
}
