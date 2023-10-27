use crate::{Style, StyleError, Styleable};
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

/// Font style attributes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontStyle {
    /// font-style: normal
    Normal,
    /// font-style: italic
    Italic,
}

impl From<FontStyle> for Style {
    fn from(value: FontStyle) -> Self {
        Style::Font(Font::Style(value))
    }
}

impl FontStyle {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            FontStyle::Normal => write!(stream, "fst-normal")?,
            FontStyle::Italic => write!(stream, "fst-italic")?,
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
            FontStyle::Normal => write!(stream, "font-style:normal")?,
            FontStyle::Italic => write!(stream, "font-style:italic")?,
        };

        Ok(())
    }
}

/// Represents the font styles.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Font {
    /// font-size: value;
    Size(i32),
    /// font-weight: value;
    Weight(i32),
    /// font-family: value;
    Family(FontFamily),
    /// font-style: value;
    Style(FontStyle),
}

impl From<Font> for Style {
    fn from(value: Font) -> Self {
        Style::Font(value)
    }
}

impl Font {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Font::Size(x) => write!(stream, "fs-{}", x)?,
            Font::Weight(x) => write!(stream, "fw-{}", x)?,
            Font::Family(x) => x.write_classname(stream)?,
            Font::Style(x) => x.write_classname(stream)?,
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
            Font::Size(x) => {
                write!(stream, "font-size:")?;
                _options.spacing(stream, *x)?;
            }
            Font::Weight(x) => {
                write!(stream, "font-weight:")?;
                _options.spacing(stream, *x)?;
            }
            Font::Family(x) => x.write_css_statement(stream, _options)?,
            Font::Style(x) => x.write_css_statement(stream, _options)?,
        };

        Ok(())
    }
}

impl<T> FontTrait for T where T: Styleable {}

/// A trait for the font attributes.
pub trait FontTrait: Styleable {
    /// Sets the font-size style attribute.
    #[inline]
    fn font_size(self, value: impl Into<i32>) -> Self {
        self.style(Font::Size(value.into()))
    }

    /// Sets the font-weight style attribute.
    #[inline]
    fn font_weight(self, value: impl Into<i32>) -> Self {
        self.style(Font::Weight(value.into()))
    }

    /// Sets the font-family style attribute.
    #[inline]
    fn font_family(self, value: impl Into<FontFamily>) -> Self {
        self.style(Font::Family(value.into()))
    }

    /// Sets the font-family style attribute to mono.
    #[inline]
    fn font_mono(self) -> Self {
        self.font_family(FontFamily::Mono)
    }

    /// Sets the font-style style attribute.
    #[inline]
    fn font_style(self, value: impl Into<FontStyle>) -> Self {
        self.style(Font::Style(value.into()))
    }
}
