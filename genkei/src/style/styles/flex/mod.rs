mod flex_direction;
mod flex_wrap;

pub use flex_direction::*;
pub use flex_wrap::*;

use crate::{Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the flex style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Flex {
    /// flex-direction: X
    Direction(FlexDirection),
    /// flex-wrap: X
    Wrap(FlexWrap),
    /// flex-grow: X
    Grow(i32),
    /// flex-shrink: X
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
            Flex::Direction(x) => x.write_classname(stream)?,
            Flex::Wrap(x) => x.write_classname(stream)?,
            Flex::Grow(x) => write!(stream, "fg-{}", x)?,
            Flex::Shrink(x) => write!(stream, "fs-{}", x)?,
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
            Flex::Direction(x) => x.write_css_statement(stream, options)?,
            Flex::Wrap(x) => x.write_css_statement(stream, options)?,
            Flex::Grow(x) => write!(stream, "flex-grow:{}", x)?,
            Flex::Shrink(x) => write!(stream, "flex-shrink:{}", x)?,
        };

        Ok(())
    }
}

impl<T> FlexTrait for T where T: Styleable {}

/// A trait for the flex style attributes.
pub trait FlexTrait: Styleable {
    #[inline]
    fn flex_direction(self, value: impl Into<FlexDirection>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn flex_direction_row(self) -> Self::Output {
        self.style(FlexDirection::Row)
    }

    #[inline]
    fn flex_direction_col(self) -> Self::Output {
        self.style(FlexDirection::Column)
    }

    #[inline]
    fn flex_wrap(self, value: impl Into<FlexWrap>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn flex_grow(self, value: impl Into<i32>) -> Self::Output {
        self.style(Flex::Grow(value.into()))
    }

    #[inline]
    fn flex_shrink(self, value: impl Into<i32>) -> Self::Output {
        self.style(Flex::Shrink(value.into()))
    }
}
