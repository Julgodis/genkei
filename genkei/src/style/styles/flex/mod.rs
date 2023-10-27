mod flex_direction;
mod flex_wrap;

pub use flex_direction::*;
pub use flex_wrap::*;

use crate::{Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the flex style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Flex {
    /// flex-direction
    Direction(FlexDirection),
    /// flex-wrap
    Wrap(FlexWrap),
    /// flex-grow
    Grow(i32),
    /// flex-shrink
    Shrink(i32),
}

impl From<Flex> for Style {
    fn from(value: Flex) -> Self {
        Style::Flex(value)
    }
}

impl Flex {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Flex::Direction(value) => value.write_classname(stream)?,
            Flex::Wrap(value) => value.write_classname(stream)?,
            Flex::Grow(value) => write!(stream, "fg-{}", value)?,
            Flex::Shrink(value) => write!(stream, "fs-{}", value)?,
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
            Flex::Direction(value) => value.write_css_statement(stream, options)?,
            Flex::Wrap(value) => value.write_css_statement(stream, options)?,
            Flex::Grow(value) => {
                write!(stream, "flex-grow:")?;
                options.spacing(stream, *value)?;
            }
            Flex::Shrink(value) => {
                write!(stream, "flex-shrink:")?;
                options.spacing(stream, *value)?;
            }
        };

        Ok(())
    }
}

impl<T> FlexTrait for T where T: Styleable {}

/// A trait for the flex style attributes.
pub trait FlexTrait: Styleable {
    /// Sets the flex style attribute.
    #[inline]
    fn flex_direction(self, value: impl Into<FlexDirection>) -> Self {
        self.style(value.into())
    }

    /// Sets the flex style attribute to row.
    #[inline]
    fn flex_direction_row(self) -> Self {
        self.style(FlexDirection::Row)
    }

    /// Sets the flex style attribute to row-reverse.
    #[inline]
    fn flex_direction_col(self) -> Self {
        self.style(FlexDirection::Column)
    }

    /// Sets the flex-wrap style attribute.
    #[inline]
    fn flex_wrap(self, value: impl Into<FlexWrap>) -> Self {
        self.style(value.into())
    }

    /// Sets the flex-grow style attribute.
    #[inline]
    fn flex_grow(self, value: impl Into<i32>) -> Self {
        self.style(Flex::Grow(value.into()))
    }

    /// Sets the flex-shrink style attribute.
    #[inline]
    fn flex_shrink(self, value: impl Into<i32>) -> Self {
        self.style(Flex::Shrink(value.into()))
    }
}
