use crate::{Grid, Style, StyleError};
use std::fmt::Write;

/// Represents the `grid-row` property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GridRow {
    /// grid-row: auto
    Auto,
    /// grid-row: span X / span X
    Span(i32),
    /// grid-row: 1 / -1
    SpanFull,
    /// grid-row-start: X
    Start(i32),
    /// grid-row-start: auto
    StartAuto,
    /// grid-row-end: X
    End(i32),
    /// grid-row-end: auto
    EndAuto,
}

impl From<GridRow> for Style {
    fn from(value: GridRow) -> Self {
        Grid::Row(value).into()
    }
}

impl GridRow {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            GridRow::Auto => write!(stream, "row-auto")?,
            GridRow::Span(x) => write!(stream, "row-span-{}", x)?,
            GridRow::SpanFull => write!(stream, "row-span-full")?,
            GridRow::Start(x) => write!(stream, "row-start-{}", x)?,
            GridRow::StartAuto => write!(stream, "row-start-auto")?,
            GridRow::End(x) => write!(stream, "row-end-{}", x)?,
            GridRow::EndAuto => write!(stream, "row-end-auto")?,
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
            GridRow::Auto => write!(stream, "grid-row:auto")?,
            GridRow::Span(x) => {
                write!(stream, "grid-row:span ")?;
                options.spacing(stream, *x)?;
                write!(stream, " / span ")?;
                options.spacing(stream, *x)?;
            }
            GridRow::SpanFull => write!(stream, "grid-row:1 / -1")?,
            GridRow::Start(x) => {
                write!(stream, "grid-row-start:")?;
                options.spacing(stream, *x)?;
            }
            GridRow::StartAuto => write!(stream, "grid-row-start:auto")?,
            GridRow::End(x) => {
                write!(stream, "grid-row-end:")?;
                options.spacing(stream, *x)?;
            }
            GridRow::EndAuto => write!(stream, "grid-row-end:auto")?,
        };

        Ok(())
    }
}
