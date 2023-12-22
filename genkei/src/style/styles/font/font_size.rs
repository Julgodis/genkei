use crate::{Font, Style, StyleError};
use std::fmt::Write;

/// The `font-size` attributes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontSize {
    /// font-size: 0.75rem;
    /// line-height: 1rem;
    Xs,
    /// font-size: 0.875rem;
    /// line-height: 1.25rem;
    Sm,
    /// font-size: 1rem;
    /// line-height: 1.5rem;
    Base,
    /// font-size: 1.125rem;
    /// line-height: 1.75rem;
    Lg,
    /// font-size: 1.25rem;
    /// line-height: 1.75rem;
    Xl,
    /// font-size: 1.5rem;
    /// line-height: 2rem;
    Xxl,
    /// font-size: 1.875rem;
    /// line-height: 2.25rem;
    Xxxl,
    /// font-size: 2.25rem;
    /// line-height: 2.5rem;
    Xxxxl,
}

impl From<FontSize> for Style {
    fn from(value: FontSize) -> Self {
        Style::Font(Font::Size(value))
    }
}

impl FontSize {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            FontSize::Xs => write!(stream, "text-xs")?,
            FontSize::Sm => write!(stream, "text-sm")?,
            FontSize::Base => write!(stream, "text-base")?,
            FontSize::Lg => write!(stream, "text-lg")?,
            FontSize::Xl => write!(stream, "text-xl")?,
            FontSize::Xxl => write!(stream, "text-2xl")?,
            FontSize::Xxxl => write!(stream, "text-3xl")?,
            FontSize::Xxxxl => write!(stream, "text-4xl")?,
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
            FontSize::Xs => write!(stream, "font-size:0.75rem;line-height:1rem")?,
            FontSize::Sm => write!(stream, "font-size:0.875rem;line-height:1.25rem")?,
            FontSize::Base => write!(stream, "font-size:1rem;line-height:1.5rem")?,
            FontSize::Lg => write!(stream, "font-size:1.125rem;line-height:1.75rem")?,
            FontSize::Xl => write!(stream, "font-size:1.25rem;line-height:1.75rem")?,
            FontSize::Xxl => write!(stream, "font-size:1.5rem;line-height:2rem")?,
            FontSize::Xxxl => write!(stream, "font-size:1.875rem;line-height:2.25rem")?,
            FontSize::Xxxxl => write!(stream, "font-size:2.25rem;line-height:2.5rem")?,
        };

        Ok(())
    }
}
