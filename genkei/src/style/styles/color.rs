use crate::{Color, Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the `color` and `background-color` style attributes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ColorStyle {
    /// color: value;
    Foreground(Color),
    /// background-color: value;
    Background(Color),
}

impl From<ColorStyle> for Style {
    fn from(value: ColorStyle) -> Self {
        Style::Color(value)
    }
}

impl ColorStyle {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            ColorStyle::Foreground(x) => {
                write!(stream, "fg-")?;
                x.write_color_name(stream)?;
            }
            ColorStyle::Background(x) => {
                write!(stream, "bg-")?;
                x.write_color_name(stream)?;
            }
        };

        Ok(())
    }

    pub(crate) fn write_css_statement<T>(
        &self,
        stream: &mut String,
        options: &T,
    ) -> Result<(), StyleError>
    where
        T: crate::StyleOptions,
    {
        match self {
            ColorStyle::Foreground(x) => {
                write!(stream, "color:")?;
                x.write_css_value(stream, options)?;
            }
            ColorStyle::Background(x) => {
                write!(stream, "background-color:")?;
                x.write_css_value(stream, options)?;
            }
        };

        Ok(())
    }
}

impl<T> ColorTrait for T where T: Styleable {}

/// A trait for the color style attribute.
pub trait ColorTrait: Styleable {
    #[inline]
    fn bg_color(self, value: impl Into<Color>) -> Self::Output {
        self.style(ColorStyle::Background(value.into()))
    }

    #[inline]
    fn fg_color(self, value: impl Into<Color>) -> Self::Output {
        self.style(ColorStyle::Foreground(value.into()))
    }

    #[inline]
    fn color(self, value: impl Into<Color>) -> Self::Output {
        self.fg_color(value)
    }
}
