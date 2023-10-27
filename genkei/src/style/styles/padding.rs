use crate::{Style, StyleOptions, Styleable, StyleError};
use std::fmt::Write;

/// Represents the padding style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Padding {
    /// padding: value;
    All(i32),
    /// padding-top: value;
    Top(i32),
    /// padding-right: value;
    Right(i32),
    /// padding-bottom: value;
    Bottom(i32),
    /// padding-left: value;
    Left(i32),
    /// padding-left: value; padding-right: value;
    X(i32),
    /// padding-top: value; padding-bottom: value;
    Y(i32),
}

impl From<Padding> for Style {
    fn from(value: Padding) -> Self {
        Style::Padding(value)
    }
}

impl Padding {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        let (name, spacing) = match self {
            Padding::All(x) => ("p", x),
            Padding::Top(x) => ("pt", x),
            Padding::Right(x) => ("pr", x),
            Padding::Bottom(x) => ("pb", x),
            Padding::Left(x) => ("pl", x),
            Padding::X(x) => ("px", x),
            Padding::Y(x) => ("py", x),
        };

        write!(stream, "{}-{}", name, spacing)?;
        Ok(())
    }

    pub(crate) fn write_css_statement<T>(
        &self,
        stream: &mut String,
        options: &T,
    ) -> Result<(), StyleError>
    where
        T: StyleOptions,
    {
        let (name, spacing) = match self {
            Padding::All(x) => ("padding", x),
            Padding::Top(x) => ("padding-top", x),
            Padding::Right(x) => ("padding-right", x),
            Padding::Bottom(x) => ("padding-bottom", x),
            Padding::Left(x) => ("padding-left", x),
            Padding::X(x) => ("padding-left", x),
            Padding::Y(x) => ("padding-top", x),
        };

        write!(stream, "{}:", name)?;
        options.spacing(stream, *spacing)?;
        Ok(())
    }
}

impl<T> PaddingTrait for T where T: Styleable {}

/// Padding style attributes.
pub trait PaddingTrait: Styleable {
    /// Sets the padding style attribute for all sides.
    #[inline]
    fn p(self, value: impl Into<i32>) -> Self {
        self.style(Padding::All(value.into()))
    }

    /// Sets the padding style attribute for the left and right sides.
    #[inline]
    fn px(self, value: impl Into<i32>) -> Self {
        self.style(Padding::X(value.into()))
    }

    /// Sets the padding style attribute for the top and bottom sides.
    #[inline]
    fn py(self, value: impl Into<i32>) -> Self {
        self.style(Padding::Y(value.into()))
    }

    /// Sets the padding style attribute for the top side.
    #[inline]
    fn pt(self, value: impl Into<i32>) -> Self {
        self.style(Padding::Top(value.into()))
    }

    /// Sets the padding style attribute for the right side.
    #[inline]
    fn pr(self, value: impl Into<i32>) -> Self {
        self.style(Padding::Right(value.into()))
    }

    /// Sets the padding style attribute for the bottom side.
    #[inline]
    fn pb(self, value: impl Into<i32>) -> Self {
        self.style(Padding::Bottom(value.into()))
    }

    /// Sets the padding style attribute for the left side.
    #[inline]
    fn pl(self, value: impl Into<i32>) -> Self {
        self.style(Padding::Left(value.into()))
    }
}
