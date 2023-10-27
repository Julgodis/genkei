use crate::{Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the backdrop-filter property style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackdropFilter {
    /// backdrop-filter: blur(value)
    Blur(i32),
}

impl From<BackdropFilter> for Style {
    fn from(value: BackdropFilter) -> Self {
        Style::BackdropFilter(value)
    }
}

impl BackdropFilter {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            BackdropFilter::Blur(x) => write!(stream, "bf-blur-{}", x)?,
        };

        Ok(())
    }

    pub(crate) fn write_css_statement<T>(
        &self,
        stream: &mut String,
        _options: &T,
    ) -> Result<(), StyleError>
    where
        T: crate::StyleOptions,
    {
        match self {
            BackdropFilter::Blur(x) => {
                write!(stream, "backdrop-filter:blur(")?;
                _options.spacing(stream, *x)?;
                write!(stream, "px)")?;
            }
        };

        Ok(())
    }
}

impl<T> BackdropFilterTrait for T where T: Styleable {}

/// A trait for the backdrop-filter style attributes.
pub trait BackdropFilterTrait: Styleable {
    /// Sets the backdrop-filter style attribute.
    #[inline]
    fn backdrop(self, value: impl Into<BackdropFilter>) -> Self {
        self.style(value.into())
    }

    /// Sets the backdrop-filter style attribute to blur.
    #[inline]
    fn backdrop_blur(self, value: impl Into<i32>) -> Self {
        self.style(BackdropFilter::Blur(value.into()))
    }
}
