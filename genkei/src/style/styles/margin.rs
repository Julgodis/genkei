use crate::{Style, StyleError, StyleOptions, Styleable};
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
        match self {
            Margin::All(x) => write!(stream, "m-{}", x)?,
            Margin::Top(x) => write!(stream, "mt-{}", x)?,
            Margin::Right(x) => write!(stream, "mr-{}", x)?,
            Margin::Bottom(x) => write!(stream, "mb-{}", x)?,
            Margin::Left(x) => write!(stream, "ml-{}", x)?,
            Margin::X(x) => write!(stream, "mx-{}", x)?,
            Margin::Y(x) => write!(stream, "my-{}", x)?,
        };

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
        match self {
            Margin::All(x) => {
                write!(stream, "margin:")?;
                options.spacing(stream, *x)?;
            }
            Margin::Top(x) => {
                write!(stream, "margin-top:")?;
                options.spacing(stream, *x)?;
            }
            Margin::Right(x) => {
                write!(stream, "margin-right:")?;
                options.spacing(stream, *x)?;
            }
            Margin::Bottom(x) => {
                write!(stream, "margin-bottom:")?;
                options.spacing(stream, *x)?;
            }
            Margin::Left(x) => {
                write!(stream, "margin-left:")?;
                options.spacing(stream, *x)?;
            }
            Margin::X(x) => {
                write!(stream, "margin-left:")?;
                options.spacing(stream, *x)?;
                write!(stream, "margin-right:")?;
                options.spacing(stream, *x)?;
            }
            Margin::Y(x) => {
                write!(stream, "margin-top:")?;
                options.spacing(stream, *x)?;
                write!(stream, "margin-bottom:")?;
                options.spacing(stream, *x)?;
            }
        };

        Ok(())
    }
}

impl<T> MarginTrait for T where T: Styleable {}

/// Margin style attributes.
pub trait MarginTrait: Styleable {
    #[inline]
    fn m(self, value: impl Into<i32>) -> Self::Output {
        self.style(Margin::All(value.into()))
    }

    #[inline]
    fn mx(self, value: impl Into<i32>) -> Self::Output {
        self.style(Margin::X(value.into()))
    }

    #[inline]
    fn my(self, value: impl Into<i32>) -> Self::Output {
        self.style(Margin::Y(value.into()))
    }

    #[inline]
    fn mt(self, value: impl Into<i32>) -> Self::Output {
        self.style(Margin::Top(value.into()))
    }

    #[inline]
    fn mr(self, value: impl Into<i32>) -> Self::Output {
        self.style(Margin::Right(value.into()))
    }

    #[inline]
    fn mb(self, value: impl Into<i32>) -> Self::Output {
        self.style(Margin::Bottom(value.into()))
    }

    #[inline]
    fn ml(self, value: impl Into<i32>) -> Self::Output {
        self.style(Margin::Left(value.into()))
    }
}
