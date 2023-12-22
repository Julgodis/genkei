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
            BackdropFilter::Blur(x) => write!(stream, "backdrop-filter:blur({}px)", x)?,
        };

        Ok(())
    }
}

impl<T> BackdropFilterTrait for T where T: Styleable {}

/// A trait for the backdrop-filter style attributes.
pub trait BackdropFilterTrait: Styleable {
    #[inline]
    fn backdrop_filter(self, value: impl Into<BackdropFilter>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn backdrop_filter_blur(self, value: impl Into<i32>) -> Self::Output {
        self.style(BackdropFilter::Blur(value.into()))
    }
}
