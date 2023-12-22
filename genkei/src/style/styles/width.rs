use crate::{Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the width style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Width {
    /// width: spacing(X);
    Value(i32),
    /// width: percentage(X, Y);
    Percent(i32, i32),
    /// width: 100%;
    Full,
    /// width: 100vw;
    Screen,
    /// width: min-content;
    MinContent,
    /// width: max-content;
    MaxContent,
    /// wdith: fit-content;
    FitContent,
}

impl From<Width> for Style {
    fn from(value: Width) -> Self {
        Style::Width(value)
    }
}

impl Width {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Width::Value(x) => write!(stream, "w-{}", x)?,
            Width::Percent(x, y) => write!(stream, "w-{}/{}", x, y)?,
            Width::Full => write!(stream, "w-full")?,
            Width::Screen => write!(stream, "w-screen")?,
            Width::MinContent => write!(stream, "w-min")?,
            Width::MaxContent => write!(stream, "w-max")?,
            Width::FitContent => write!(stream, "w-fit")?,
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
            Width::Value(x) => {
                write!(stream, "width:")?;
                options.spacing(stream, *x)?;
            }
            Width::Percent(x, y) => {
                write!(stream, "width:")?;
                options.percentage(stream, *x, *y)?;
            }
            Width::Full => write!(stream, "width:100%")?,
            Width::Screen => write!(stream, "width:100vw")?,
            Width::MinContent => write!(stream, "width:min-content")?,
            Width::MaxContent => write!(stream, "width:max-content")?,
            Width::FitContent => write!(stream, "width:fit-content")?,
        };

        Ok(())
    }
}

/// Represents the min-width style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MinWidth {
    /// min-width: spacing(X);
    Value(i32),
    /// min-width: 100%;
    Full,
    /// min-width: min-content;
    MinContent,
    /// min-width: max-content;
    MaxContent,
    /// min-wdith: fit-content;
    FitContent,
}

impl From<MinWidth> for Style {
    fn from(value: MinWidth) -> Self {
        Style::MinWidth(value)
    }
}

impl MinWidth {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            MinWidth::Value(x) => write!(stream, "min-w-{}", x)?,
            MinWidth::Full => write!(stream, "min-w-full")?,
            MinWidth::MinContent => write!(stream, "min-w-min")?,
            MinWidth::MaxContent => write!(stream, "min-w-max")?,
            MinWidth::FitContent => write!(stream, "min-w-fit")?,
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
            MinWidth::Value(x) => {
                write!(stream, "min-width:")?;
                options.spacing(stream, *x)?;
            }
            MinWidth::Full => write!(stream, "min-width:100%")?,
            MinWidth::MinContent => write!(stream, "min-width:min-content")?,
            MinWidth::MaxContent => write!(stream, "min-width:max-content")?,
            MinWidth::FitContent => write!(stream, "min-width:fit-content")?,
        };

        Ok(())
    }
}

impl<T> WidthTrait for T where T: Styleable {}

/// Width style attributes.
pub trait WidthTrait: Styleable {
    /// width: spacing(X);
    #[inline]
    fn w(self, value: impl Into<i32>) -> Self::Output {
        self.style(Width::Value(value.into()))
    }

    /// width: percentage(X, Y);
    #[inline]
    fn w_percent(self, x: impl Into<i32>, y: impl Into<i32>) -> Self::Output {
        self.style(Width::Percent(x.into(), y.into()))
    }

    /// width: 100vw;
    #[inline]
    fn w_screen(self) -> Self::Output {
        self.style(Width::Screen)
    }

    /// width: 100%;
    #[inline]
    fn w_full(self) -> Self::Output {
        self.style(Width::Full)
    }

    /// width: fit-content;
    #[inline]
    fn w_fit(self) -> Self::Output {
        self.style(Width::FitContent)
    }

    /// width: min-content;
    #[inline]
    fn w_min(self) -> Self::Output {
        self.style(Width::MinContent)
    }

    /// width: max-content;
    #[inline]
    fn w_max(self) -> Self::Output {
        self.style(Width::MaxContent)
    }

    /// min-width: spacing(X);
    #[inline]
    fn min_w(self, value: impl Into<i32>) -> Self::Output {
        self.style(MinWidth::Value(value.into()))
    }

    /// min-width: 100%;
    #[inline]
    fn min_w_full(self) -> Self::Output {
        self.style(MinWidth::Full)
    }

    /// min-width: fit-content;
    #[inline]
    fn min_w_fit(self) -> Self::Output {
        self.style(MinWidth::FitContent)
    }

    /// min-width: min-content;
    #[inline]
    fn min_w_min(self) -> Self::Output {
        self.style(MinWidth::MinContent)
    }

    /// min-width: max-content;
    #[inline]
    fn min_w_max(self) -> Self::Output {
        self.style(MinWidth::MaxContent)
    }
}
