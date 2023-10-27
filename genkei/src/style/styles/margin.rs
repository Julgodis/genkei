use crate::{Style, Styleable, StyleOptions, StyleError};
use std::fmt::Write;

/// Represents the margin style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Margin {
    /// margin: value;
    All(i32),
    /// margin-top: value;
    Top(i32),
    /// margin-right: value;
    Right(i32),
    /// margin-bottom: value;
    Bottom(i32),
    /// margin-left: value;
    Left(i32),
    /// margin-left: value; margin-right: value;
    X(i32),
    /// margin-top: value; margin-bottom: value;
    Y(i32),
}

impl From<Margin> for Style {
    fn from(value: Margin) -> Self {
        Style::Margin(value)
    }
}

impl Margin {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        let (name, spacing) = match self {
            Margin::All(x) => ("m", x),
            Margin::Top(x) => ("mt", x),
            Margin::Right(x) => ("mr", x),
            Margin::Bottom(x) => ("mb", x),
            Margin::Left(x) => ("ml", x),
            Margin::X(x) => ("mx", x),
            Margin::Y(x) => ("my", x),
        };

        write!(stream, "{}-{}", name, spacing)?;
        Ok(())
    }

    pub(crate) fn write_css_statement<T>(&self, stream: &mut String, options: &T) -> Result<(), StyleError>
    where
        T: StyleOptions,
    {
        let (name, spacing) = match self {
            Margin::All(x) => ("margin", x),
            Margin::Top(x) => ("margin-top", x),
            Margin::Right(x) => ("margin-right", x),
            Margin::Bottom(x) => ("margin-bottom", x),
            Margin::Left(x) => ("margin-left", x),
            Margin::X(x) => ("margin-left", x),
            Margin::Y(x) => ("margin-top", x),
        };

        write!(stream, "{}:", name)?;
        options.spacing(stream, *spacing)?;
        Ok(())
    }
}

impl<T> MarginTrait for T where T: Styleable {}

/// Margin style attributes.
pub trait MarginTrait: Styleable {
    /// Sets the margin style attribute for all sides.
    #[inline]
    fn m(self, value: impl Into<i32>) -> Self {
        self.style(Margin::All(value.into()))
    }

    /// Sets the margin style attribute for the left and right sides.
    #[inline]
    fn mx(self, value: impl Into<i32>) -> Self {
        self.style(Margin::X(value.into()))
    }

    /// Sets the margin style attribute for the top and bottom sides.
    #[inline]
    fn my(self, value: impl Into<i32>) -> Self {
        self.style(Margin::Y(value.into()))
    }

    /// Sets the margin style attribute for the top side.
    #[inline]
    fn mt(self, value: impl Into<i32>) -> Self {
        self.style(Margin::Top(value.into()))
    }

    /// Sets the margin style attribute for the right side.
    #[inline]
    fn mr(self, value: impl Into<i32>) -> Self {
        self.style(Margin::Right(value.into()))
    }

    /// Sets the margin style attribute for the bottom side.
    #[inline]
    fn mb(self, value: impl Into<i32>) -> Self {
        self.style(Margin::Bottom(value.into()))
    }

    /// Sets the margin style attribute for the left side.
    #[inline]
    fn ml(self, value: impl Into<i32>) -> Self {
        self.style(Margin::Left(value.into()))
    }
}
