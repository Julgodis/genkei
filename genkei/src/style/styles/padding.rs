use crate::{Style, StyleError, StyleOptions, Styleable};
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
        match self {
            Padding::All(x) => write!(stream, "p-{}", x)?,
            Padding::Top(x) => write!(stream, "pt-{}", x)?,
            Padding::Right(x) => write!(stream, "pr-{}", x)?,
            Padding::Bottom(x) => write!(stream, "pb-{}", x)?,
            Padding::Left(x) => write!(stream, "pl-{}", x)?,
            Padding::X(x) => write!(stream, "px-{}", x)?,
            Padding::Y(x) => write!(stream, "py-{}", x)?,
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
            Padding::All(x) => {
                write!(stream, "padding:")?;
                options.spacing(stream, *x)?;
            }
            Padding::Top(x) => {
                write!(stream, "padding-top:")?;
                options.spacing(stream, *x)?;
            }
            Padding::Right(x) => {
                write!(stream, "padding-right:")?;
                options.spacing(stream, *x)?;
            }
            Padding::Bottom(x) => {
                write!(stream, "padding-bottom:")?;
                options.spacing(stream, *x)?;
            }
            Padding::Left(x) => {
                write!(stream, "padding-left:")?;
                options.spacing(stream, *x)?;
            }
            Padding::X(x) => {
                write!(stream, "padding-left:")?;
                options.spacing(stream, *x)?;
                write!(stream, ";padding-right:")?;
                options.spacing(stream, *x)?;
            }
            Padding::Y(x) => {
                write!(stream, "padding-top:")?;
                options.spacing(stream, *x)?;
                write!(stream, ";padding-bottom:")?;
                options.spacing(stream, *x)?;
            }
        };

        Ok(())
    }
}

impl<T> PaddingTrait for T where T: Styleable {}

/// Padding style attributes.
pub trait PaddingTrait: Styleable {
    #[inline]
    fn p(self, value: impl Into<i32>) -> Self::Output {
        self.style(Padding::All(value.into()))
    }

    #[inline]
    fn px(self, value: impl Into<i32>) -> Self::Output {
        self.style(Padding::X(value.into()))
    }

    #[inline]
    fn py(self, value: impl Into<i32>) -> Self::Output {
        self.style(Padding::Y(value.into()))
    }

    #[inline]
    fn pt(self, value: impl Into<i32>) -> Self::Output {
        self.style(Padding::Top(value.into()))
    }

    #[inline]
    fn pr(self, value: impl Into<i32>) -> Self::Output {
        self.style(Padding::Right(value.into()))
    }

    #[inline]
    fn pb(self, value: impl Into<i32>) -> Self::Output {
        self.style(Padding::Bottom(value.into()))
    }

    #[inline]
    fn pl(self, value: impl Into<i32>) -> Self::Output {
        self.style(Padding::Left(value.into()))
    }
}
