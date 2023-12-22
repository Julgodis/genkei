use crate::Style;
use std::collections::BTreeSet;
use std::fmt::Write;

/// Specifies the style options.
pub trait StyleOptions: Default {
    /// Specifies the spacing unit in css units, e.g. `1` is `0.25rem`.
    fn spacing<Stream>(&self, stream: &mut Stream, value: i32) -> Result<(), StyleError>
    where
        Stream: std::fmt::Write;

    /// Specifies the percentage value ratio, e.g. `1/5` is `20%`.
    fn percentage<Stream>(&self, stream: &mut Stream, x: i32, y: i32) -> Result<(), StyleError>
    where
        Stream: std::fmt::Write;

    /// Specifices the border unit in css units, e.g. `1` is `1px`.
    fn border<Stream>(&self, stream: &mut Stream, value: i32) -> Result<(), StyleError>
    where
        Stream: std::fmt::Write;

    /// Specifices the font size unit in css units, e.g. `1` is `0.25rem`.
    fn font_size<Stream>(&self, stream: &mut Stream, value: i32) -> Result<(), StyleError>
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

    fn border<Stream>(&self, stream: &mut Stream, value: i32) -> Result<(), StyleError>
    where
        Stream: std::fmt::Write,
    {
        write!(stream, "{}px", value)?;
        Ok(())
    }

    fn font_size<Stream>(&self, stream: &mut Stream, value: i32) -> Result<(), StyleError>
    where
        Stream: std::fmt::Write,
    {
        write!(stream, "{}rem", value as f32 / 4.0)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum StyleError {
    /// Error occurred while format writing to the buffer.
    FormatError,
    /// The style cannot be rendered as a css statement.
    CssStatementUnsupported(Style),
    /// The style cannot be rendered as a css selector.
    CssSelectorUnsupported(Style),
    /// The style cannot be rendered as a css classname.
    CssClassnameUnsupported(Style),
    /// The style cannot be rendered as an inline style.
    InlineStylingNotSupported(Style),
}

impl std::fmt::Display for StyleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleError::FormatError => write!(f, "format error"),
            StyleError::CssStatementUnsupported(style) => {
                write!(f, "css statement unsupported: {:?}", style)
            }
            StyleError::CssSelectorUnsupported(style) => {
                write!(f, "css selector unsupported: {:?}", style)
            }
            StyleError::CssClassnameUnsupported(style) => {
                write!(f, "css classname unsupported: {:?}", style)
            }
            StyleError::InlineStylingNotSupported(style) => {
                write!(f, "inline styling not supported: {:?}", style)
            }
        }
    }
}

impl std::error::Error for StyleError {}

impl From<std::fmt::Error> for StyleError {
    fn from(_: std::fmt::Error) -> Self {
        Self::FormatError
    }
}

/// A renderer for styles. This renderer is used to generate css from styles.
#[derive(Debug, Clone)]
pub struct StyleRenderer<Opt: StyleOptions> {
    options: Opt,
    capacity: usize,
    include_css_reset: bool,
    styles: BTreeSet<Style>,
}

impl<Opt: StyleOptions> StyleRenderer<Opt> {
    pub fn new(include_css_reset: bool) -> Self {
        Self::with_options(Opt::default(), include_css_reset, 4096)
    }

    pub fn with_options(options: Opt, include_css_reset: bool, capacity: usize) -> Self {
        Self {
            options,
            capacity,
            include_css_reset,
            styles: BTreeSet::new(),
        }
    }

    fn write_css_statement(
        style: &Style,
        stream: &mut String,
        options: &Opt,
    ) -> Result<(), StyleError> {
        match style {
            Style::Padding(x) => x.write_css_statement(stream, options)?,
            Style::Margin(x) => x.write_css_statement(stream, options)?,
            Style::Width(x) => x.write_css_statement(stream, options)?,
            Style::MinWidth(x) => x.write_css_statement(stream, options)?,
            Style::Height(x) => x.write_css_statement(stream, options)?,
            Style::MinHeight(x) => x.write_css_statement(stream, options)?,
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
            Style::Gap(x) => x.write_css_statement(stream, options)?,

            Style::State(x) => Self::write_css_statement(&x.inner, stream, options)?,
            Style::MediaQuery(_, x) => Self::write_css_statement(x, stream, options)?,
            Style::DataQuery(_, x) => Self::write_css_statement(x, stream, options)?,
            _ => return Err(StyleError::CssStatementUnsupported(style.clone())),
        }

        Ok(())
    }

    fn write_css_selector(style: &Style, stream: &mut String) -> Result<(), StyleError> {
        match style {
            Style::Padding(x) => x.write_classname(stream)?,
            Style::Margin(x) => x.write_classname(stream)?,
            Style::Width(x) => x.write_classname(stream)?,
            Style::MinWidth(x) => x.write_classname(stream)?,
            Style::Height(x) => x.write_classname(stream)?,
            Style::MinHeight(x) => x.write_classname(stream)?,
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
            Style::Gap(x) => x.write_classname(stream)?,

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
            Style::DataQuery(dq, inner) => {
                write_data_query_prefix(stream, dq)?;
                stream.push_str("\\:");
                Self::write_css_selector(inner, stream)?;
                write_data_query_suffix(stream, dq)?;
            }
            _ => return Err(StyleError::CssSelectorUnsupported(style.clone())),
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
            Style::MinWidth(x) => x.write_classname(stream)?,
            Style::Height(x) => x.write_classname(stream)?,
            Style::MinHeight(x) => x.write_classname(stream)?,
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
            Style::Gap(x) => x.write_classname(stream)?,

            Style::State(x) => {
                x.write_classname_prefix(stream)?;
                Self::write_css_selector(&x.inner, stream)?;
            }
            Style::MediaQuery(mq, inner) => {
                mq.write_classname(stream)?;
                stream.push(':');
                Self::write_css_selector(inner, stream)?;
            }
            Style::DataQuery(dq, inner) => {
                write_data_query_classname(stream, dq)?;
                stream.push(':');
                Self::write_css_selector(inner, stream)?;
            }
            _ => return Err(StyleError::CssClassnameUnsupported(style.clone())),
        }

        Ok(())
    }

    fn write_style(style: &Style, stream: &mut String, options: &Opt) -> Result<(), StyleError> {
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

    pub fn include_style(&mut self, style: impl Into<Style>) {
        self.styles.insert(style.into());
    }

    pub fn include_styles(&mut self, styles: impl IntoIterator<Item = impl Into<Style>>) {
        for style in styles {
            self.styles.insert(style.into());
        }
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

    pub fn use_as_inline_style(&mut self, styles: BTreeSet<Style>) -> Result<String, StyleError> {
        let mut buffer = String::with_capacity(128);
        for style in styles {
            match style {
                Style::State(_) => return Err(StyleError::InlineStylingNotSupported(style)),
                Style::MediaQuery(_, _) => {
                    return Err(StyleError::InlineStylingNotSupported(style))
                }
                _ => {
                    if !buffer.is_empty() {
                        buffer.push(';');
                    }
                    Self::write_css_statement(&style, &mut buffer, &self.options)?;
                }
            }
        }

        Ok(buffer)
    }

    pub fn render(self) -> Result<(String, BTreeSet<Style>), StyleError> {
        let Self {
            styles,
            capacity,
            include_css_reset,
            options,
            ..
        } = self;

        let mut buffer = String::with_capacity(capacity);
        if include_css_reset {
            buffer.push_str(include_str!("../../../reset.css"));
        }

        for style in &styles {
            Self::write_style(style, &mut buffer, &options)?;
        }

        Ok((buffer, styles))
    }

    pub fn to_css(style: impl Into<Style>) -> Result<String, StyleError> {
        let mut renderer = StyleRenderer::<DefaultStyleOptions>::new(false);
        renderer.include_style(style);
        Ok(renderer.render()?.0)
    }
}

fn write_data_query_classname(stream: &mut String, dq: &str) -> Result<(), StyleError> {
    stream.push('[');
    stream.push_str(dq);
    stream.push(']');
    Ok(())
}

fn write_data_query_prefix(stream: &mut String, dq: &str) -> Result<(), StyleError> {
    stream.push('\\');
    stream.push('[');
    for ch in dq.chars() {
        // escape characters not supported in css classnames
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => stream.push(ch),
            _ => write!(stream, "\\{:x}", ch as u32)?,
        }
    }
    stream.push('\\');
    stream.push(']');
    Ok(())
}

fn write_data_query_suffix(stream: &mut String, dq: &str) -> Result<(), StyleError> {
    stream.push('[');
    for ch in dq.chars() {
        // escape characters not supported in css classnames
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => stream.push(ch),
            _ => write!(stream, "\\{:x}", ch as u32)?,
        }
    }
    stream.push(']');
    Ok(())
}
