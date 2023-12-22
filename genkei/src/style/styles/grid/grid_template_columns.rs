use crate::{Grid, Style, StyleError};
use std::fmt::Write;

/// Represents the `grid-template-columns` property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GridTemplateColumns {
    /// grid-template-columns: repeat(X, minmax(0, 1fr))
    Repeat(i32),
    /// grid-template-columns: none
    None,
}

impl From<GridTemplateColumns> for Style {
    fn from(value: GridTemplateColumns) -> Self {
        Grid::TemplateColumns(value).into()
    }
}

impl GridTemplateColumns {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            GridTemplateColumns::Repeat(x) => write!(stream, "grid-cols-{}", x)?,
            GridTemplateColumns::None => write!(stream, "grid-cols-none")?,
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
            GridTemplateColumns::Repeat(x) => {
                write!(stream, "grid-template-columns:repeat({},minmax(0,1fr))", x)?
            }
            GridTemplateColumns::None => write!(stream, "grid-template-columns:none")?,
        };

        Ok(())
    }
}
