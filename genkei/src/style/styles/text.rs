use crate::{Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the text alignment style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TextAlign {
    /// text-align: left;
    Left,
    /// text-align: center;
    Center,
    /// text-align: right;
    Right,
}

impl From<TextAlign> for Style {
    fn from(value: TextAlign) -> Self {
        value.into()
    }
}

impl TextAlign {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            TextAlign::Left => write!(stream, "text-left")?,
            TextAlign::Center => write!(stream, "text-center")?,
            TextAlign::Right => write!(stream, "text-right")?,
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
            TextAlign::Left => write!(stream, "text-align:left")?,
            TextAlign::Center => write!(stream, "text-align:center")?,
            TextAlign::Right => write!(stream, "text-align:right")?,
        };

        Ok(())
    }
}

impl<T> TextTrait for T where T: Styleable {}

/// Text style attributes.
pub trait TextTrait: Styleable {
    #[inline]
    fn text_align(self, value: impl Into<TextAlign>) -> Self::Output {
        self.style(Style::TextAlign(value.into()))
    }

    #[inline]
    fn text_center(self) -> Self::Output {
        self.text_align(TextAlign::Center)
    }

    #[inline]
    fn text_left(self) -> Self::Output {
        self.text_align(TextAlign::Left)
    }

    #[inline]
    fn text_right(self) -> Self::Output {
        self.text_align(TextAlign::Right)
    }
}
