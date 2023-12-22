use crate::{Grid, Style, StyleError};
use std::fmt::Write;

/// Represents the `grid-template-rows` property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GridTemplateRows {
    /// grid-template-rows: repeat(X, minmax(0, 1fr))
    Repeat(i32),
    /// grid-template-rows: none
    None,
}

impl From<GridTemplateRows> for Style {
    fn from(value: GridTemplateRows) -> Self {
        Grid::TemplateRows(value).into()
    }
}

impl GridTemplateRows {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            GridTemplateRows::Repeat(x) => write!(stream, "grid-rows-{}", x)?,
            GridTemplateRows::None => write!(stream, "grid-rows-none")?,
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
            GridTemplateRows::Repeat(x) => {
                write!(stream, "grid-template-rows:repeat({},minmax(0,1fr))", x)?
            }
            GridTemplateRows::None => write!(stream, "grid-template-rows:none")?,
        };

        Ok(())
    }
}
