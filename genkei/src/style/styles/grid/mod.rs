mod grid_column;
mod grid_row;
mod grid_template_columns;
mod grid_template_rows;

pub use grid_column::*;
pub use grid_row::*;
pub use grid_template_columns::*;
pub use grid_template_rows::*;

use crate::{Style, StyleError, Styleable};

/// Represents the `grid` properties.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Grid {
    /// grid-template-columns: value
    TemplateColumns(GridTemplateColumns),
    /// grid-template-rows: value
    TemplateRows(GridTemplateRows),
    /// grid-column: value
    Column(GridColumn),
    /// grid-row: value
    Row(GridRow),
}

impl From<Grid> for Style {
    fn from(value: Grid) -> Self {
        Style::Grid(value)
    }
}

impl Grid {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Grid::TemplateColumns(value) => value.write_classname(stream),
            Grid::TemplateRows(value) => value.write_classname(stream),
            Grid::Column(value) => value.write_classname(stream),
            Grid::Row(value) => value.write_classname(stream),
        }
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
            Grid::TemplateColumns(value) => value.write_css_statement(stream, options),
            Grid::TemplateRows(value) => value.write_css_statement(stream, options),
            Grid::Column(value) => value.write_css_statement(stream, options),
            Grid::Row(value) => value.write_css_statement(stream, options),
        }
    }
}

impl<T> GridTrait for T where T: Styleable {}

/// Grid style attributes.
pub trait GridTrait: Styleable {
    #[inline]
    fn grid_cols(self, value: impl Into<i32>) -> Self::Output {
        self.style(GridTemplateColumns::Repeat(value.into()))
    }

    #[inline]
    fn grid_rows(self, value: impl Into<i32>) -> Self::Output {
        self.style(GridTemplateRows::Repeat(value.into()))
    }

    #[inline]
    fn col_span(self, value: impl Into<i32>) -> Self::Output {
        self.style(GridColumn::Span(value.into()))
    }

    #[inline]
    fn row_span(self, value: impl Into<i32>) -> Self::Output {
        self.style(GridRow::Span(value.into()))
    }
}
