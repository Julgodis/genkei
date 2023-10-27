mod align_content;
mod align_items;
mod align_self;

pub use align_content::*;
pub use align_items::*;
pub use align_self::*;

use crate::{Style, StyleError};

/// Represents the align styles.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Align {
    /// align-content: value
    Content(AlignContent),
    /// align-items: value
    Items(AlignItems),
    /// align-self: value
    Self_(AlignSelf),
}

impl From<Align> for Style {
    fn from(value: Align) -> Self {
        Style::Align(value)
    }
}

impl Align {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Align::Content(value) => value.write_classname(stream),
            Align::Items(value) => value.write_classname(stream),
            Align::Self_(value) => value.write_classname(stream),
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
            Align::Content(value) => value.write_css_statement(stream, options),
            Align::Items(value) => value.write_css_statement(stream, options),
            Align::Self_(value) => value.write_css_statement(stream, options),
        }
    }
}
