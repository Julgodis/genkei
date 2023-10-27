use crate::{Grid, Style, StyleError};
use std::fmt::Write;

/// Represents the `grid-column` property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GridColumn {
    /// grid-column: auto
    Auto,
    /// grid-column: span X / span X
    Span(i32),
    /// grid-column: 1 / -1
    SpanFull,
    /// grid-column-start: X
    Start(i32),
    /// grid-column-start: auto
    StartAuto,
    /// grid-column-end: X
    End(i32),
    /// grid-column-end: auto
    EndAuto,
}

impl From<GridColumn> for Style {
    fn from(value: GridColumn) -> Self {
        Grid::Column(value).into()
    }
}

impl GridColumn {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            GridColumn::Auto => write!(stream, "col-auto")?,
            GridColumn::Span(x) => write!(stream, "col-span-{}", x)?,
            GridColumn::SpanFull => write!(stream, "col-span-full")?,
            GridColumn::Start(x) => write!(stream, "col-start-{}", x)?,
            GridColumn::StartAuto => write!(stream, "col-start-auto")?,
            GridColumn::End(x) => write!(stream, "col-end-{}", x)?,
            GridColumn::EndAuto => write!(stream, "col-end-auto")?,
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
            GridColumn::Auto => write!(stream, "grid-column:auto")?,
            GridColumn::Span(x) => {
                write!(stream, "grid-column:span ")?;
                options.spacing(stream, *x)?;
                write!(stream, " / span ")?;
                options.spacing(stream, *x)?;
            }
            GridColumn::SpanFull => write!(stream, "grid-column:1 / -1")?,
            GridColumn::Start(x) => {
                write!(stream, "grid-column-start:")?;
                options.spacing(stream, *x)?;
            }
            GridColumn::StartAuto => write!(stream, "grid-column-start:auto")?,
            GridColumn::End(x) => {
                write!(stream, "grid-column-end:")?;
                options.spacing(stream, *x)?;
            }
            GridColumn::EndAuto => write!(stream, "grid-column-end:auto")?,
        };

        Ok(())
    }
}
