use super::Tag;
use std::{collections::BTreeSet, error::Error};

#[derive(Debug, Clone)]
pub struct RenderResult {
    html: String,
    #[cfg(feature = "style")]
    css: String,
    #[cfg(feature = "style")]
    styles: BTreeSet<crate::Style>,
}

impl RenderResult {
    pub fn html(&mut self) -> String {
        std::mem::take(&mut self.html)
    }

    #[cfg(feature = "style")]
    pub fn css(&mut self) -> String {
        std::mem::take(&mut self.css)
    }

    #[cfg(feature = "style")]
    pub fn styles(&mut self) -> BTreeSet<crate::Style> {
        std::mem::take(&mut self.styles)
    }

    pub fn parts(self) -> (String, String) {
        if cfg!(feature = "style") {
            (self.html, self.css)
        } else {
            (self.html, String::new())
        }
    }
}

#[derive(Debug, Clone)]
pub enum RenderError {
    #[cfg(feature = "style")]
    Style(crate::StyleError),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "style")]
            Self::Style(error) => write!(f, "Style error: {}", error),
        }
    }
}

impl Error for RenderError {}

#[cfg(feature = "style")]
impl From<crate::StyleError> for RenderError {
    fn from(value: crate::StyleError) -> Self {
        Self::Style(value)
    }
}

#[derive(Debug, Clone)]
pub struct Renderer {
    buffer: String,
    #[cfg(feature = "style")]
    style_renderer: crate::StyleRenderer<crate::DefaultStyleOptions>,
    #[cfg(feature = "style")]
    style_or_classname: bool,
    tags: Vec<Tag>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            buffer: String::with_capacity(1024),
            #[cfg(feature = "style")]
            style_renderer: crate::StyleRenderer::new(true),
            #[cfg(feature = "style")]
            style_or_classname: false,
            tags: Vec::new(),
        }
    }

    #[cfg(feature = "style")]
    pub fn use_classname(&mut self) {
        self.style_or_classname = false;
    }

    #[cfg(feature = "style")]
    pub fn use_inline_style(&mut self) {
        self.style_or_classname = true;
    }

    #[inline]
    pub fn begin_tag(&mut self) {
        self.buffer.push('<');
    }

    #[inline]
    pub fn begin_tag_with_slash(&mut self) {
        self.buffer.push_str("</");
    }

    #[inline]
    pub fn end_tag(&mut self) {
        self.buffer.push('>');
    }

    #[inline]
    pub fn end_tag_with_slash(&mut self) {
        // if the last character is not a whitespace or " or ' or >, then add a whitespace
        if let Some(last) = self.buffer.chars().last() {
            if !matches!(last, ' ' | '"' | '\'' | '>') {
                self.buffer.push(' ');
            }
        }
        self.buffer.push_str("/>");
    }

    #[inline]
    pub fn push_char(&mut self, c: char) {
        self.buffer.push(c);
    }

    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    #[inline]
    fn escape_attribute_value(&mut self, value: &str) {
        if value.is_empty()
            || value
                .chars()
                .any(|c| matches!(c, '"' | '\'' | '<' | '>' | '&' | '´' | '=' | ' '))
        {
            self.escape_attribute_value_quoted(value)
        } else {
            self.escape_attribute_value_unquoted(value)
        }
    }

    #[inline]
    fn escape_attribute_value_quoted(&mut self, value: &str) {
        self.push_char('"');
        self.escape_attribute_value_unquoted(value);
        self.push_char('"');
    }

    fn escape_attribute_value_unquoted(&mut self, value: &str) {
        for c in value.chars() {
            match c {
                '"' => self.push_str("&quot;"),
                '\'' => self.push_str("&apos;"),
                '<' => self.push_str("&lt;"),
                '>' => self.push_str("&gt;"),
                '&' => self.push_str("&amp;"),
                _ => self.push_char(c),
            }
        }
    }

    pub fn push_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    pub fn render(mut self) -> Result<RenderResult, RenderError> {
        let tags = std::mem::take(&mut self.tags);
        for tag in tags {
            tag.render(&mut self)?;
        }

        let (css, styles) = self.style_renderer.render()?;

        Ok(RenderResult {
            html: self.buffer,
            #[cfg(feature = "style")]
            css,
            #[cfg(feature = "style")]
            styles,
        })
    }

    pub fn render_tag(tag: Tag) -> Result<RenderResult, RenderError> {
        let mut renderer = Self::new();
        tag.render(&mut renderer)?;
        Ok(RenderResult {
            html: renderer.buffer,
            #[cfg(feature = "style")]
            css: renderer.style_renderer.render()?.0,
            #[cfg(feature = "style")]
            styles: BTreeSet::new(),
        })
    }
}

impl Tag {
    fn render(self, renderer: &mut Renderer) -> Result<(), RenderError> {
        let Self {
            tag_name,
            mut attributes,
            children,
            text_content,
            inner_html,
            #[cfg(feature = "style")]
            styles,
            self_closable,
            omit_end_slash,
        } = self;

        renderer.push_char('<');
        renderer.push_str(&tag_name);

        #[cfg(feature = "style")]
        if !styles.is_empty() {
            let (attribute_name, attribute_value) = if renderer.style_or_classname {
                (
                    "style",
                    renderer.style_renderer.use_as_inline_style(styles)?,
                )
            } else {
                ("class", renderer.style_renderer.use_as_classname(styles)?)
            };

            attributes
                .entry(attribute_name.into())
                .and_modify(|existing_class| {
                    if let Some(class) = existing_class {
                        let class = class.to_mut();
                        class.push(' ');
                        class.push_str(&attribute_value);
                    }
                })
                .or_insert(Some(attribute_value.into()));
        }

        for (key, value) in attributes {
            renderer.push_char(' ');
            renderer.push_str(&key);
            if let Some(value) = value {
                renderer.push_char('=');
                renderer.escape_attribute_value(&value);
            }
        }

        let has_children = !children.is_empty() || text_content.is_some() || inner_html.is_some();
        let self_closable = self_closable && !has_children;
        if self_closable {
            if omit_end_slash {
                renderer.end_tag();
            } else {
                renderer.end_tag_with_slash();
            }
        } else {
            renderer.end_tag();
            if let Some(inner_html) = inner_html {
                renderer.push_str(&inner_html);
            }

            for child in children {
                child.render(renderer)?;
            }

            if let Some(text_content) = text_content {
                // TODO: escape text content
                renderer.push_str(&text_content);
            }

            renderer.begin_tag_with_slash();
            renderer.push_str(&tag_name);
            renderer.end_tag();
        }

        Ok(())
    }

    pub fn to_html(self) -> Result<String, RenderError> {
        Renderer::render_tag(self).map(|result| result.html)
    }
}
