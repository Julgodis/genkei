use crate::{Color, Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the border-width styles.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BorderWidth {
    /// border-width: value
    All(i32),
    /// border-left-width: value
    /// border-right-width: value
    X(i32),
    /// border-top-width: value
    /// border-bottom-width: value
    Y(i32),
    /// border-top-width: value;
    Top(i32),
    /// border-right-width: value;
    Right(i32),
    /// border-bottom-width: value;
    Bottom(i32),
    /// border-left-width: value;
    Left(i32),
}

impl From<BorderWidth> for Style {
    fn from(value: BorderWidth) -> Self {
        Border::Width(value).into()
    }
}

impl BorderWidth {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            BorderWidth::All(x) => write!(stream, "bw-{}", x)?,
            BorderWidth::X(x) => write!(stream, "bw-x-{}", x)?,
            BorderWidth::Y(x) => write!(stream, "bw-y-{}", x)?,
            BorderWidth::Top(x) => write!(stream, "bw-top-{}", x)?,
            BorderWidth::Right(x) => write!(stream, "bw-right-{}", x)?,
            BorderWidth::Bottom(x) => write!(stream, "bw-bottom-{}", x)?,
            BorderWidth::Left(x) => write!(stream, "bw-left-{}", x)?,
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
            BorderWidth::All(x) => {
                write!(stream, "border-width:")?;
                options.border(stream, *x)?;
            }
            BorderWidth::X(x) => {
                write!(stream, "border-left-width:")?;
                options.border(stream, *x)?;
                write!(stream, ";border-right-width:")?;
                options.border(stream, *x)?;
            }
            BorderWidth::Y(x) => {
                write!(stream, "border-top-width:")?;
                options.border(stream, *x)?;
                write!(stream, ";border-bottom-width:")?;
                options.border(stream, *x)?;
            }
            BorderWidth::Top(x) => {
                write!(stream, "border-top-width:")?;
                options.border(stream, *x)?;
            }
            BorderWidth::Right(x) => {
                write!(stream, "border-right-width:")?;
                options.border(stream, *x)?;
            }
            BorderWidth::Bottom(x) => {
                write!(stream, "border-bottom-width:")?;
                options.border(stream, *x)?;
            }
            BorderWidth::Left(x) => {
                write!(stream, "border-left-width:")?;
                options.border(stream, *x)?;
            }
        };

        Ok(())
    }
}

/// Represents the border styles.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Border {
    /// border-width: value;
    Width(BorderWidth),
    /// border-color: value;
    Color(Color),
    /// border-radius: value;
    Radius(i32),
}

impl From<Border> for Style {
    fn from(value: Border) -> Self {
        Style::Border(value)
    }
}

impl Border {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Border::Width(x) => x.write_classname(stream)?,
            Border::Color(x) => {
                write!(stream, "bc-")?;
                x.write_color_name(stream)?;
            }
            Border::Radius(x) => write!(stream, "br-{}", x)?,
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
            Border::Width(x) => x.write_css_statement(stream, options)?,
            Border::Color(x) => {
                write!(stream, "border-color:")?;
                x.write_css_value(stream, options)?;
            }
            Border::Radius(x) => {
                write!(stream, "border-radius:")?;
                options.spacing(stream, *x)?;
            }
        };

        Ok(())
    }
}

impl<T> BorderTrait for T where T: Styleable {}

/// A trait for the border style attribute.
pub trait BorderTrait: Styleable {
    #[inline]
    fn border(self, value: impl Into<i32>) -> Self::Output {
        self.style(BorderWidth::All(value.into()))
    }

    #[inline]
    fn border_x(self, value: impl Into<i32>) -> Self::Output {
        self.style(BorderWidth::X(value.into()))
    }

    #[inline]
    fn border_y(self, value: impl Into<i32>) -> Self::Output {
        self.style(BorderWidth::Y(value.into()))
    }

    #[inline]
    fn border_top(self, value: impl Into<i32>) -> Self::Output {
        self.style(BorderWidth::Top(value.into()))
    }

    #[inline]
    fn border_right(self, value: impl Into<i32>) -> Self::Output {
        self.style(BorderWidth::Right(value.into()))
    }

    #[inline]
    fn border_bottom(self, value: impl Into<i32>) -> Self::Output {
        self.style(BorderWidth::Bottom(value.into()))
    }

    #[inline]
    fn border_left(self, value: impl Into<i32>) -> Self::Output {
        self.style(BorderWidth::Left(value.into()))
    }

    #[inline]
    fn border_color(self, value: impl Into<Color>) -> Self::Output {
        self.style(Border::Color(value.into()))
    }

    #[inline]
    fn border_radius(self, value: impl Into<i32>) -> Self::Output {
        self.style(Border::Radius(value.into()))
    }
}
