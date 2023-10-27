use crate::{Flex, Style, StyleError};
use std::fmt::Write;

/// Represents the flex direction style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FlexDirection {
    /// flex-direction: row
    Row,
    /// flex-direction: row-reverse
    RowReverse,
    /// flex-direction: column
    Column,
    /// flex-direction: column-reverse
    ColumnReverse,
}

impl From<FlexDirection> for Style {
    fn from(value: FlexDirection) -> Self {
        Style::Flex(Flex::Direction(value))
    }
}

impl FlexDirection {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            FlexDirection::Row => write!(stream, "fxd-row")?,
            FlexDirection::RowReverse => write!(stream, "fxd-row-reverse")?,
            FlexDirection::Column => write!(stream, "fxd-col")?,
            FlexDirection::ColumnReverse => write!(stream, "fxd-col-reverse")?,
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
            FlexDirection::Row => write!(stream, "flex-direction:row")?,
            FlexDirection::RowReverse => write!(stream, "flex-direction:row-reverse")?,
            FlexDirection::Column => write!(stream, "flex-direction:column")?,
            FlexDirection::ColumnReverse => write!(stream, "flex-direction:column-reverse")?,
        };

        Ok(())
    }
}
