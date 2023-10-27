use crate::{Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the height style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Height {
    /// height: value;
    Value(i32),
    /// height: percent;
    Percent(i32, i32),
    /// height: 100%;
    Full,
    /// height: 100vh;
    Screen,
    /// height: min-content;
    MinContent,
    /// height: max-content;
    MaxContent,
    /// height: fit-content;
    FitContent,
}

impl From<Height> for Style {
    fn from(value: Height) -> Self {
        Style::Height(value)
    }
}

impl Height {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Height::Value(x) => write!(stream, "h-{}", x)?,
            Height::Percent(x, y) => write!(stream, "h-{}/{}", x, y)?,
            Height::Full => write!(stream, "h-full")?,
            Height::Screen => write!(stream, "h-screen")?,
            Height::MinContent => write!(stream, "h-min")?,
            Height::MaxContent => write!(stream, "h-max")?,
            Height::FitContent => write!(stream, "h-fit")?,
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
            Height::Value(x) => {
                write!(stream, "height:")?;
                options.spacing(stream, *x)?
            }
            Height::Percent(x, y) => {
                write!(stream, "height:")?;
                options.percentage(stream, *x, *y)?
            }
            Height::Full => write!(stream, "height:100%")?,
            Height::Screen => write!(stream, "height:100vh")?,
            Height::MinContent => write!(stream, "height:min-content")?,
            Height::MaxContent => write!(stream, "height:max-content")?,
            Height::FitContent => write!(stream, "height:fit-content")?,
        };

        Ok(())
    }
}

impl<T> HeightTrait for T where T: Styleable {}

/// Height style attributes.
pub trait HeightTrait: Styleable {
    /// Sets the height style attribute.
    #[inline]
    fn h(self, value: impl Into<i32>) -> Self {
        self.style(Height::Value(value.into()))
    }

    /// Sets the height style attribute to 100%.
    #[inline]
    fn h_full(self) -> Self {
        self.style(Height::Full)
    }
}
