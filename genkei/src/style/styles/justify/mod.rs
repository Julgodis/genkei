mod justify_content;
mod justify_items;
mod justify_self;

pub use justify_content::*;
pub use justify_items::*;
pub use justify_self::*;

use crate::{Style, StyleError};

/// Represents the justify styles.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Justify {
    /// justify-content: value
    Content(JustifyContent),
    /// justify-items: value
    Items(JustifyItems),
    /// justify-self: value
    Self_(JustifySelf),
}

impl From<Justify> for Style {
    fn from(value: Justify) -> Self {
        Style::Justify(value)
    }
}

impl Justify {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Justify::Content(value) => value.write_classname(stream),
            Justify::Items(value) => value.write_classname(stream),
            Justify::Self_(value) => value.write_classname(stream),
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
            Justify::Content(value) => value.write_css_statement(stream, options),
            Justify::Items(value) => value.write_css_statement(stream, options),
            Justify::Self_(value) => value.write_css_statement(stream, options),
        }
    }
}
