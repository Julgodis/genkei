use crate::{Justify, Style, StyleError};
use std::fmt::Write;

/// Represents the justify-content style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JustifyContent {
    /// justify-content: normal
    Normal,
    /// justify-content: flex-start
    FlexStart,
    /// justify-content: flex-end
    FlexEnd,
    /// justify-content: center
    Center,
    /// justify-content: space-between
    SpaceBetween,
    /// justify-content: space-around
    SpaceAround,
    /// justify-content: space-evenly
    SpaceEvenly,
    /// justify-content: stretch
    Stretch,
}

impl From<JustifyContent> for Style {
    fn from(value: JustifyContent) -> Self {
        Justify::Content(value).into()
    }
}

impl JustifyContent {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            JustifyContent::Normal => write!(stream, "justify-normal")?,
            JustifyContent::FlexStart => write!(stream, "justify-start")?,
            JustifyContent::FlexEnd => write!(stream, "justify-end")?,
            JustifyContent::Center => write!(stream, "justify-center")?,
            JustifyContent::SpaceBetween => write!(stream, "justify-between")?,
            JustifyContent::SpaceAround => write!(stream, "justify-around")?,
            JustifyContent::SpaceEvenly => write!(stream, "justify-evenly")?,
            JustifyContent::Stretch => write!(stream, "justify-stretch")?,
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
            JustifyContent::Normal => write!(stream, "justify-content:normal")?,
            JustifyContent::FlexStart => write!(stream, "justify-content:flex-start")?,
            JustifyContent::FlexEnd => write!(stream, "justify-content:flex-end")?,
            JustifyContent::Center => write!(stream, "justify-content:center")?,
            JustifyContent::SpaceBetween => write!(stream, "justify-content:space-between")?,
            JustifyContent::SpaceAround => write!(stream, "justify-content:space-around")?,
            JustifyContent::SpaceEvenly => write!(stream, "justify-content:space-evenly")?,
            JustifyContent::Stretch => write!(stream, "justify-content:stretch")?,
        };

        Ok(())
    }
}
