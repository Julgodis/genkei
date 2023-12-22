mod outline_style;

pub use outline_style::*;

use crate::{Color, Style, StyleError, Styleable, StyleBuilder};
use std::fmt::Write;

/// Represents the outline style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Outline {
    /// outline-style: value
    Style(OutlineStyle),
    /// outline-width: value
    Width(i32),
    /// outline-color: value
    Color(Color),
}

impl From<Outline> for Style {
    fn from(value: Outline) -> Self {
        Style::Outline(value)
    }
}

impl Outline {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Outline::Style(value) => value.write_classname(stream)?,
            Outline::Width(value) => write!(stream, "ow-{}", value)?,
            Outline::Color(value) => {
                write!(stream, "oc-")?;
                value.write_color_name(stream)?;
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
            Outline::Style(value) => value.write_css_statement(stream, options)?,
            Outline::Width(value) => {
                write!(stream, "outline-width:")?;
                options.border(stream, *value)?;
            }
            Outline::Color(value) => {
                write!(stream, "outline-color:")?;
                value.write_css_value(stream, options)?;
            }
        };

        Ok(())
    }
}

impl<T> OutlineTrait for T where T: Styleable {}

/// A trait for the outline style attributes.
pub trait OutlineTrait: Styleable {
    #[inline]
    fn outline_style(self, value: impl Into<OutlineStyle>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn outline_width(self, value: impl Into<i32>) -> Self::Output {
        self.style(Outline::Width(value.into()))
    }

    #[inline]
    fn outline_color(self, value: impl Into<Color>) -> Self::Output {
        self.style(Outline::Color(value.into()))
    }

    #[inline]
    fn outline(
        self,
        style: impl Into<OutlineStyle>,
        width: impl Into<i32>,
        color: impl Into<Color>,
    ) -> Self::Output {
        self.styles(
            StyleBuilder::new()
                .outline_style(style)
                .outline_width(width)
                .outline_color(color)
                .build()
                .into_iter(),
        )
    }
}
