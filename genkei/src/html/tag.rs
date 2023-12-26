use crate::{html, Attribute, Attributes, Children, IntoAttribute, Str, TextContent};
use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "style")]
use crate::style::Style;

#[derive(Debug, Clone)]
pub struct Tag {
    pub tag_name: Str,
    pub attributes: BTreeMap<Str, Option<Str>>,
    pub children: Vec<Tag>,
    pub text_content: Option<Str>,
    pub inner_html: Option<Str>,
    #[cfg(feature = "style")]
    pub styles: BTreeSet<Style>,
    pub self_closable: bool,
    pub omit_end_slash: bool,
}

impl Tag {
    pub fn new(tag_name: impl Into<Str>) -> Self {
        Self {
            tag_name: tag_name.into(),
            attributes: BTreeMap::new(),
            children: Vec::new(),
            text_content: None,
            inner_html: None,
            #[cfg(feature = "style")]
            styles: BTreeSet::new(),
            self_closable: false,
            omit_end_slash: false,
        }
    }

    pub fn new_with_options(tag_name: impl Into<Str>, self_closable: bool) -> Self {
        Self {
            tag_name: tag_name.into(),
            attributes: BTreeMap::new(),
            children: Vec::new(),
            text_content: None,
            inner_html: None,
            #[cfg(feature = "style")]
            styles: BTreeSet::new(),
            self_closable,
            omit_end_slash: false,
        }
    }

    pub fn inner_html(mut self, inner_html: impl Into<Str>) -> Self {
        self.inner_html = Some(inner_html.into());
        self
    }
}

impl Attributes for Tag {
    type Output = Self;

    fn attr(mut self, attribute: impl IntoAttribute) -> Self {
        let attribute = attribute.into_attribute();
        match attribute {
            Attribute::Key(key) => {
                self.attributes.insert(key, None);
            }
            Attribute::KeyValue(key, value) => {
                self.attributes.insert(key, Some(value));
            }
        }
        self
    }
}

// HTML tags can have children.
impl Children for Tag {
    type Output = Self;
    type Child = Self;

    fn child(mut self, child: impl Into<Self::Child>) -> Self {
        self.children.push(child.into());
        self
    }

    fn children(mut self, children: impl IntoIterator<Item = impl Into<Self::Child>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }
}

// HTML tags can have text content.
impl TextContent for Tag {
    type Output = Self;

    fn text_content(mut self, text_content: impl Into<Str>) -> Self {
        self.text_content = Some(text_content.into());
        self
    }
}

// HTML tags have HTML attributes.
impl html::HtmlAttribute for Tag {}

#[cfg(feature = "style")]
impl crate::style::Styleable for Tag {
    type Output = Self;

    fn style_raw(mut self, style: Style) -> Self {
        self.styles.insert(style);
        self
    }

    fn styles_raw(mut self, styles: impl IntoIterator<Item = Style>) -> Self {
        self.styles.extend(styles);
        self
    }
}
