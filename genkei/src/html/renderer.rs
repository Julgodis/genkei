use super::Tag;

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
            style_renderer: crate::StyleRenderer::new(),
            #[cfg(feature = "style")]
            style_or_classname: false,
            tags: Vec::new(),
        }
    }

    #[cfg(feature = "style")]
    pub fn styling_as_classname(mut self) -> Self {
        self.style_or_classname = false;
        self
    }

    #[cfg(feature = "style")]
    pub fn styling_as_style(mut self) -> Self {
        self.style_or_classname = true;
        self
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

    pub fn render(mut self) -> String {
        let tags = std::mem::take(&mut self.tags);
        for tag in tags {
            tag.render(&mut self);
        }

        self.buffer
    }

    pub fn render_tag(tag: Tag) -> String {
        let mut renderer = Self::new();
        tag.render(&mut renderer);
        renderer.buffer
    }
}

impl Tag {
    fn render(self, renderer: &mut Renderer) {
        let Self {
            tag_name,
            mut attributes,
            children,
            text_content,
            #[cfg(feature = "style")]
            styles,
            self_closable,
            omit_end_slash,
        } = self;

        renderer.push_char('<');
        renderer.push_str(&tag_name);

        #[cfg(feature = "style")]
        if !styles.is_empty() {
            let (attribute_name, style_classes) = if renderer.style_or_classname {
                (
                    "style",
                    renderer
                        .style_renderer
                        .use_as_style(styles)
                        .expect("Failed to render styles."),
                )
            } else {
                (
                    "class",
                    renderer
                        .style_renderer
                        .use_as_classname(styles)
                        .expect("Failed to render styles."),
                )
            };

            attributes
                .entry(attribute_name.into())
                .and_modify(|existing_class| {
                    if let Some(class) = existing_class {
                        let class = class.to_mut();
                        class.push(' ');
                        class.push_str(&style_classes);
                    }
                })
                .or_insert(Some(style_classes.into()));
        }

        for (key, value) in attributes {
            renderer.push_char(' ');
            renderer.push_str(&key);
            if let Some(value) = value {
                renderer.push_char('=');
                renderer.escape_attribute_value(&value);
            }
        }

        let has_children = !children.is_empty() || text_content.is_some();
        let self_closable = self_closable && !has_children;
        if self_closable {
            if omit_end_slash {
                renderer.end_tag();
            } else {
                renderer.end_tag_with_slash();
            }
        } else {
            renderer.end_tag();
            for child in children {
                child.render(renderer);
            }

            if let Some(text_content) = text_content {
                renderer.push_str(&text_content);
            }

            renderer.begin_tag_with_slash();
            renderer.push_str(&tag_name);
            renderer.end_tag();
        }
    }

    pub fn to_html(self) -> String {
        Renderer::render_tag(self)
    }
}
