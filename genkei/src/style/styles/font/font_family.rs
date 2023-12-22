
use crate::{Style, StyleError, Font};
use std::fmt::Write;

/// Font style attributes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontFamily {
    /// Use the sans-serif font family.
    Sans,
    /// Use the serif font family.
    Serif,
    /// Use the mono font family.
    Mono,
}

impl From<FontFamily> for Style {
    fn from(value: FontFamily) -> Self {
        Style::Font(Font::Family(value))
    }
}

impl FontFamily {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            FontFamily::Sans => write!(stream, "ff-sans")?,
            FontFamily::Serif => write!(stream, "ff-serif")?,
            FontFamily::Mono => write!(stream, "ff-mono")?,
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
        write!(stream, "font-family:{}", self.to_css_value())?;
        Ok(())
    }

    pub fn to_css_value(&self) -> &'static str {
        match self {
            FontFamily::Sans => {
                r#"ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,"Helvetica Neue",Arial,"Noto Sans",sans-serif,"Apple Color Emoji","Segoe UI Emoji","Segoe UI Symbol","Noto Color Emoji""#
            }
            FontFamily::Serif => r#"ui-serif,Georgia,Cambria,"Times New Roman",Times,serif"#,
            FontFamily::Mono => {
                r#"ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,"Liberation Mono","Courier New",monospace"#
            }
        }
    }
}
