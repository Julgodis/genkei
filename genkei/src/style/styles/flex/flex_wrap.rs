use crate::{Flex, Style, StyleError};
use std::fmt::Write;

/// Represents the flex wrap style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FlexWrap {
    /// flex-wrap: nowrap
    NoWrap,
    /// flex-wrap: wrap
    Wrap,
    /// flex-wrap: wrap-reverse
    WrapReverse,
}

impl From<FlexWrap> for Style {
    fn from(value: FlexWrap) -> Self {
        Style::Flex(Flex::Wrap(value))
    }
}

impl FlexWrap {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            FlexWrap::NoWrap => write!(stream, "fxw-nowrap")?,
            FlexWrap::Wrap => write!(stream, "fxw-wrap")?,
            FlexWrap::WrapReverse => write!(stream, "fxw-wrap-reverse")?,
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
            FlexWrap::NoWrap => write!(stream, "flex-wrap:nowrap")?,
            FlexWrap::Wrap => write!(stream, "flex-wrap:wrap")?,
            FlexWrap::WrapReverse => write!(stream, "flex-wrap:wrap-reverse")?,
        };

        Ok(())
    }
}
