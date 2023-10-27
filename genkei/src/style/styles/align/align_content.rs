use crate::{Align, Style, StyleError};
use std::fmt::Write;

/// Represents the align-content style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AlignContent {
    /// align-content: normal
    Normal,
    /// align-content: center
    Center,
    /// align-content: flex-start
    FlexStart,
    /// align-content: flex-end
    FlexEnd,
    /// align-content: space-between
    SpaceBetween,
    /// align-content: space-around
    SpaceAround,
    /// align-content: space-evenly
    SpaceEvenly,
    /// align-content: baseline
    Baseline,
    /// align-content: stretch
    Stretch,
}

impl From<AlignContent> for Style {
    fn from(value: AlignContent) -> Self {
        Align::Content(value).into()
    }
}

impl AlignContent {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            AlignContent::Normal => write!(stream, "content-normal")?,
            AlignContent::Center => write!(stream, "content-center")?,
            AlignContent::FlexStart => write!(stream, "content-start")?,
            AlignContent::FlexEnd => write!(stream, "content-end")?,
            AlignContent::SpaceBetween => write!(stream, "content-between")?,
            AlignContent::SpaceAround => write!(stream, "content-around")?,
            AlignContent::SpaceEvenly => write!(stream, "content-evenly")?,
            AlignContent::Baseline => write!(stream, "content-baseline")?,
            AlignContent::Stretch => write!(stream, "content-stretch")?,
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
            AlignContent::Normal => write!(stream, "align-content:normal")?,
            AlignContent::Center => write!(stream, "align-content:center")?,
            AlignContent::FlexStart => write!(stream, "align-content:flex-start")?,
            AlignContent::FlexEnd => write!(stream, "align-content:flex-end")?,
            AlignContent::SpaceBetween => write!(stream, "align-content:space-between")?,
            AlignContent::SpaceAround => write!(stream, "align-content:space-around")?,
            AlignContent::SpaceEvenly => write!(stream, "align-content:space-evenly")?,
            AlignContent::Baseline => write!(stream, "align-content:baseline")?,
            AlignContent::Stretch => write!(stream, "align-content:stretch")?,
        };

        Ok(())
    }
}
