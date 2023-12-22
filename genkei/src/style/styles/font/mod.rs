mod font_family;
mod font_size;
mod font_style;

pub use font_family::*;
pub use font_size::*;
pub use font_style::*;

use crate::{Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the font styles.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Font {
    /// font-size: X;
    Size(FontSize),
    /// font-weight: X;
    Weight(i32),
    /// font-family: X;
    Family(FontFamily),
    /// font-style: X;
    Style(FontStyle),
}

impl From<Font> for Style {
    fn from(value: Font) -> Self {
        Style::Font(value)
    }
}

impl Font {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Font::Size(x) => x.write_classname(stream)?,
            Font::Weight(x) => write!(stream, "fw-{}", x)?,
            Font::Family(x) => x.write_classname(stream)?,
            Font::Style(x) => x.write_classname(stream)?,
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
            Font::Size(x) => x.write_css_statement(stream, options)?,
            Font::Weight(x) => {
                write!(stream, "font-weight:{}", x)?;
            }
            Font::Family(x) => x.write_css_statement(stream, options)?,
            Font::Style(x) => x.write_css_statement(stream, options)?,
        };

        Ok(())
    }
}

impl<T> FontTrait for T where T: Styleable {}

/// A trait for the font attributes.
pub trait FontTrait: Styleable {
    #[inline]
    fn font_size(self, value: impl Into<FontSize>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn text_xs(self) -> Self::Output {
        self.font_size(FontSize::Xs)
    }

    #[inline]
    fn text_sm(self) -> Self::Output {
        self.font_size(FontSize::Sm)
    }

    #[inline]
    fn text_base(self) -> Self::Output {
        self.font_size(FontSize::Base)
    }

    #[inline]
    fn text_lg(self) -> Self::Output {
        self.font_size(FontSize::Lg)
    }

    #[inline]
    fn text_xl(self) -> Self::Output {
        self.font_size(FontSize::Xl)
    }

    #[inline]
    fn text_2xl(self) -> Self::Output {
        self.font_size(FontSize::Xxl)
    }

    #[inline]
    fn text_3xl(self) -> Self::Output {
        self.font_size(FontSize::Xxxl)
    }

    #[inline]
    fn text_4xl(self) -> Self::Output {
        self.font_size(FontSize::Xxxxl)
    }

    #[inline]
    fn font_weight(self, value: impl Into<i32>) -> Self::Output {
        self.style(Font::Weight(value.into()))
    }

    #[inline]
    fn font_family(self, value: impl Into<FontFamily>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn font_mono(self) -> Self::Output {
        self.font_family(FontFamily::Mono)
    }

    #[inline]
    fn font_style(self, value: impl Into<FontStyle>) -> Self::Output {
        self.style(value.into())
    }
}
