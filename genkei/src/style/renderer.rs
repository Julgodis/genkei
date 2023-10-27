use crate::{Str, Style};
use std::collections::BTreeSet;

/// Specifies the style options.
pub trait StyleOptions {
    /// Specifies the spacing unit in css units, e.g. `1` is `0.25rem`.
    fn spacing<Stream>(&self, stream: &mut Stream, value: i32) -> Result<(), StyleError>
    where
        Stream: std::fmt::Write;

    /// Specifies the percentage value ratio, e.g. `1/5` is `20%`.
    fn percentage<Stream>(&self, stream: &mut Stream, x: i32, y: i32) -> Result<(), StyleError>
    where
        Stream: std::fmt::Write;
}

#[derive(Debug, Clone, Default)]
pub struct DefaultStyleOptions;

impl StyleOptions for DefaultStyleOptions {
    fn spacing<Stream>(&self, stream: &mut Stream, value: i32) -> Result<(), StyleError>
    where
        Stream: std::fmt::Write,
    {
        write!(stream, "{}rem", value as f32 / 4.0)?;
        Ok(())
    }

    fn percentage<Stream>(&self, stream: &mut Stream, x: i32, y: i32) -> Result<(), StyleError>
    where
        Stream: std::fmt::Write,
    {
        write!(stream, "{}%", (x as f32 / y as f32) * 100.0)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum StyleError {
    /// Error occurred while format writing to the buffer.
    FormatError,
    /// The style cannot be rendered.
    StyleCannotBeRendered,
}

impl From<std::fmt::Error> for StyleError {
    fn from(_: std::fmt::Error) -> Self {
        Self::FormatError
    }
}

#[derive(Debug, Clone)]
pub struct StyleRenderer<Options> {
    styles: BTreeSet<Style>,
    options: Options,
    buffer: String,
}

impl<Options> Default for StyleRenderer<Options>
where
    Options: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Options> StyleRenderer<Options>
where
    Options: Default,
{
    pub fn new() -> Self {
        Self {
            styles: BTreeSet::new(),
            options: Options::default(),
            buffer: String::with_capacity(1024),
        }
    }

    /// Include a style to the renderer.
    pub fn include_style(&mut self, style: impl Into<Style>) {
        self.styles.insert(style.into());
    }

    /// Include styles to the renderer.
    pub fn include_styles(&mut self, styles: impl IntoIterator<Item = impl Into<Style>>) {
        self.styles.extend(styles.into_iter().map(|x| x.into()));
    }

    pub fn buffer(self) -> String {
        self.buffer
    }
}

impl<Options> StyleRenderer<Options>
where
    Options: StyleOptions + Default,
{
    fn write_css_statement(
        style: &Style,
        stream: &mut String,
        options: &Options,
    ) -> Result<(), StyleError> {
        match style {
            Style::Padding(x) => x.write_css_statement(stream, options)?,
            Style::Margin(x) => x.write_css_statement(stream, options)?,
            Style::Width(x) => x.write_css_statement(stream, options)?,
            Style::Height(x) => x.write_css_statement(stream, options)?,
            Style::Color(x) => x.write_css_statement(stream, options)?,
            Style::Font(x) => x.write_css_statement(stream, options)?,
            Style::TextAlign(x) => x.write_css_statement(stream, options)?,
            Style::Display(x) => x.write_css_statement(stream, options)?,
            Style::Flex(x) => x.write_css_statement(stream, options)?,
            Style::Justify(x) => x.write_css_statement(stream, options)?,
            Style::Align(x) => x.write_css_statement(stream, options)?,
            Style::Border(x) => x.write_css_statement(stream, options)?,
            Style::Outline(x) => x.write_css_statement(stream, options)?,
            Style::Grid(x) => x.write_css_statement(stream, options)?,
            Style::Cursor(x) => x.write_css_statement(stream, options)?,
            Style::BackdropFilter(x) => x.write_css_statement(stream, options)?,

            Style::State(x) => Self::write_css_statement(&x.inner, stream, options)?,
            Style::MediaQuery(_, x) => Self::write_css_statement(x, stream, options)?,
            _ => return Err(StyleError::StyleCannotBeRendered),
        }

        Ok(())
    }

    fn write_css_selector(style: &Style, stream: &mut String) -> Result<(), StyleError> {
        match style {
            Style::Padding(x) => x.write_classname(stream)?,
            Style::Margin(x) => x.write_classname(stream)?,
            Style::Width(x) => x.write_classname(stream)?,
            Style::Height(x) => x.write_classname(stream)?,
            Style::Color(x) => x.write_classname(stream)?,
            Style::Font(x) => x.write_classname(stream)?,
            Style::TextAlign(x) => x.write_classname(stream)?,
            Style::Display(x) => x.write_classname(stream)?,
            Style::Flex(x) => x.write_classname(stream)?,
            Style::Justify(x) => x.write_classname(stream)?,
            Style::Align(x) => x.write_classname(stream)?,
            Style::Border(x) => x.write_classname(stream)?,
            Style::Outline(x) => x.write_classname(stream)?,
            Style::Grid(x) => x.write_classname(stream)?,
            Style::Cursor(x) => x.write_classname(stream)?,
            Style::BackdropFilter(x) => x.write_classname(stream)?,

            Style::State(x) => {
                x.write_selector_prefix(stream)?;
                Self::write_css_selector(&x.inner, stream)?;
                x.write_selector_suffix(stream)?;
            }
            Style::MediaQuery(mq, inner) => {
                mq.write_classname(stream)?;
                stream.push_str("\\:");
                Self::write_css_selector(inner, stream)?;
            }
            _ => return Err(StyleError::StyleCannotBeRendered),
        }

        Ok(())
    }

    fn write_css_selector_root(style: &Style, stream: &mut String) -> Result<(), StyleError> {
        stream.push('.');
        Self::write_css_selector(style, stream)?;
        Ok(())
    }

    fn write_classname(style: &Style, stream: &mut String) -> Result<(), StyleError> {
        match style {
            Style::Padding(x) => x.write_classname(stream)?,
            Style::Margin(x) => x.write_classname(stream)?,
            Style::Width(x) => x.write_classname(stream)?,
            Style::Height(x) => x.write_classname(stream)?,
            Style::Color(x) => x.write_classname(stream)?,
            Style::Font(x) => x.write_classname(stream)?,
            Style::TextAlign(x) => x.write_classname(stream)?,
            Style::Display(x) => x.write_classname(stream)?,
            Style::Flex(x) => x.write_classname(stream)?,
            Style::Justify(x) => x.write_classname(stream)?,
            Style::Align(x) => x.write_classname(stream)?,
            Style::Border(x) => x.write_classname(stream)?,
            Style::Outline(x) => x.write_classname(stream)?,
            Style::Grid(x) => x.write_classname(stream)?,
            Style::Cursor(x) => x.write_classname(stream)?,
            Style::BackdropFilter(x) => x.write_classname(stream)?,

            Style::State(x) => {
                x.write_classname_prefix(stream)?;
                Self::write_css_selector(&x.inner, stream)?;
            }
            Style::MediaQuery(mq, inner) => {
                mq.write_classname(stream)?;
                stream.push(':');
                Self::write_css_selector(inner, stream)?;
            }
            _ => return Err(StyleError::StyleCannotBeRendered),
        }

        Ok(())
    }

    fn write_style(
        style: &Style,
        stream: &mut String,
        options: &Options,
    ) -> Result<(), StyleError> {
        match style {
            Style::MediaQuery(mq, _) => {
                // TODO: media-queries can be grouped as an optimization.
                mq.write_selector(stream)?;
                stream.push('{');
                Self::write_css_selector_root(style, stream)?;
                stream.push('{');
                Self::write_css_statement(style, stream, options)?;
                stream.push('}');
                stream.push('}');
            }
            style => {
                Self::write_css_selector_root(style, stream)?;
                stream.push('{');
                Self::write_css_statement(style, stream, options)?;
                stream.push('}');
            }
        }

        Ok(())
    }

    pub fn render(self) -> Result<String, StyleError> {
        let Self {
            styles,
            options,
            mut buffer,
            ..
        } = self;

        for style in styles {
            if !buffer.is_empty() {
                buffer.push(';');
            }
            Self::write_style(&style, &mut buffer, &options)?;
        }

        Ok(buffer)
    }

    pub fn use_as_classname(&mut self, styles: BTreeSet<Style>) -> Result<String, StyleError> {
        let mut buffer = String::with_capacity(128);
        for style in styles {
            if !buffer.is_empty() {
                buffer.push(' ');
            }
            Self::write_classname(&style, &mut buffer)?;
            self.include_style(style);
        }

        Ok(buffer)
    }

    pub fn use_as_style(&mut self, styles: BTreeSet<Style>) -> Result<String, StyleError> {
        let mut buffer = String::with_capacity(128);
        for style in styles {
            match style {
                Style::State(_) => return Err(StyleError::StyleCannotBeRendered),
                Style::MediaQuery(_, _) => return Err(StyleError::StyleCannotBeRendered),
                _ => {
                    if !buffer.is_empty() {
                        buffer.push(';');
                    }
                    Self::write_css_statement(&style, &mut buffer, &self.options)?;
                    self.include_style(style);
                }
            }
        }

        Ok(buffer)
    }

    /// Generate css statement from the style using the default renderer with default options.
    pub fn to_css(style: &Style) -> Result<Str, StyleError> {
        let mut renderer = Self::new();
        Self::write_style(style, &mut renderer.buffer, &renderer.options)?;
        Ok(renderer.buffer().into())
    }
}

impl Style {
    pub fn to_css(&self) -> Result<Str, StyleError> {
        StyleRenderer::<DefaultStyleOptions>::to_css(self)
    }
}
