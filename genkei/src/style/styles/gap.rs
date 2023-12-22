use crate::{Style, Styleable};
use std::fmt::Write;

/// Represents the `gap` style attribute.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Gap {
    /// gap: value;
    Value(i32),
    /// column-gap: value;
    Column(i32),
    /// row-gap: value;
    Row(i32),
}

impl From<Gap> for Style {
    fn from(value: Gap) -> Self {
        Style::Gap(value)
    }
}

impl Gap {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), crate::StyleError> {
        match self {
            Gap::Value(x) => write!(stream, "gap-{}", x)?,
            Gap::Column(x) => write!(stream, "column-gap-{}", x)?,
            Gap::Row(x) => write!(stream, "row-gap-{}", x)?,
        };

        Ok(())
    }

    pub(crate) fn write_css_statement<T>(
        &self,
        stream: &mut String,
        options: &T,
    ) -> Result<(), crate::StyleError>
    where
        T: crate::StyleOptions,
    {
        match self {
            Gap::Value(_) => write!(stream, "gap:")?,
            Gap::Column(_) => write!(stream, "column-gap:")?,
            Gap::Row(_) => write!(stream, "row-gap:")?,
        };

        options.spacing(
            stream,
            match self {
                Gap::Value(x) => *x,
                Gap::Column(x) => *x,
                Gap::Row(x) => *x,
            },
        )?;

        Ok(())
    }
}

impl<T> GapTrait for T where T: Styleable {}

/// Gap style attributes.
pub trait GapTrait: Styleable {
    #[inline]
    fn gap(self, value: impl Into<i32>) -> Self::Output {
        self.style(Gap::Value(value.into()))
    }

    #[inline]
    fn column_gap(self, value: impl Into<i32>) -> Self::Output {
        self.style(Gap::Column(value.into()))
    }

    #[inline]
    fn row_gap(self, value: impl Into<i32>) -> Self::Output {
        self.style(Gap::Row(value.into()))
    }
}
