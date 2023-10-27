use crate::{Align, Style, StyleError};
use std::fmt::Write;

/// Represents the align-items style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AlignItems {
    /// align-items: flex-start
    FlexStart,
    /// align-items: flex-end
    FlexEnd,
    /// align-items: center
    Center,
    /// align-items: baseline
    Baseline,
    /// align-items: stretch
    Stretch,
}

impl From<AlignItems> for Style {
    fn from(value: AlignItems) -> Self {
        Align::Items(value).into()
    }
}

impl AlignItems {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            AlignItems::FlexStart => write!(stream, "items-start")?,
            AlignItems::FlexEnd => write!(stream, "items-end")?,
            AlignItems::Center => write!(stream, "items-center")?,
            AlignItems::Baseline => write!(stream, "items-baseline")?,
            AlignItems::Stretch => write!(stream, "items-stretch")?,
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
            AlignItems::FlexStart => write!(stream, "align-items:flex-start")?,
            AlignItems::FlexEnd => write!(stream, "align-items:flex-end")?,
            AlignItems::Center => write!(stream, "align-items:center")?,
            AlignItems::Baseline => write!(stream, "align-items:baseline")?,
            AlignItems::Stretch => write!(stream, "align-items:stretch")?,
        };

        Ok(())
    }
}
