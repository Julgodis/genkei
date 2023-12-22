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

/// Represents the min-height style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MinHeight {
    /// height: value;
    Value(i32),
    /// height: 100%;
    Full,
    /// height: min-content;
    MinContent,
    /// height: max-content;
    MaxContent,
    /// height: fit-content;
    FitContent,
}

impl From<MinHeight> for Style {
    fn from(value: MinHeight) -> Self {
        Style::MinHeight(value)
    }
}

impl MinHeight {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            MinHeight::Value(x) => write!(stream, "min-h-{}", x)?,
            MinHeight::Full => write!(stream, "min-h-full")?,
            MinHeight::MinContent => write!(stream, "min-h-min")?,
            MinHeight::MaxContent => write!(stream, "min-h-max")?,
            MinHeight::FitContent => write!(stream, "min-h-fit")?,
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
            MinHeight::Value(x) => {
                write!(stream, "min-height:")?;
                options.spacing(stream, *x)?
            }
            MinHeight::Full => write!(stream, "min-height:100%")?,
            MinHeight::MinContent => write!(stream, "min-height:min-content")?,
            MinHeight::MaxContent => write!(stream, "min-height:max-content")?,
            MinHeight::FitContent => write!(stream, "min-height:fit-content")?,
        };

        Ok(())
    }
}

impl<T> HeightTrait for T where T: Styleable {}

/// Height style attributes.
pub trait HeightTrait: Styleable {
    #[inline]
    fn h(self, value: impl Into<i32>) -> Self::Output {
        self.style(Height::Value(value.into()))
    }

    #[inline]
    fn h_percent(self, x: impl Into<i32>, y: impl Into<i32>) -> Self::Output {
        self.style(Height::Percent(x.into(), y.into()))
    }

    #[inline]
    fn h_full(self) -> Self::Output {
        self.style(Height::Full)
    }

    #[inline]
    fn h_screen(self) -> Self::Output {
        self.style(Height::Screen)
    }

    #[inline]
    fn h_min_content(self) -> Self::Output {
        self.style(Height::MinContent)
    }

    #[inline]
    fn h_max_content(self) -> Self::Output {
        self.style(Height::MaxContent)
    }

    #[inline]
    fn h_fit_content(self) -> Self::Output {
        self.style(Height::FitContent)
    }

    #[inline]
    fn min_h(self, value: impl Into<i32>) -> Self::Output {
        self.style(MinHeight::Value(value.into()))
    }

    #[inline]
    fn min_h_full(self) -> Self::Output {
        self.style(MinHeight::Full)
    }

    #[inline]
    fn min_h_min_content(self) -> Self::Output {
        self.style(MinHeight::MinContent)
    }

    #[inline]
    fn min_h_max_content(self) -> Self::Output {
        self.style(MinHeight::MaxContent)
    }

    #[inline]
    fn min_h_fit_content(self) -> Self::Output {
        self.style(MinHeight::FitContent)
    }
}
