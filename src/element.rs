use std::{borrow::Cow, collections::HashSet};

use crate::style::{Style, AttributeExtension};

pub type Str = Cow<'static, str>;

pub struct Element {
    pub tag_name: Str,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Element>,
    pub text_content: Option<String>,
    pub styles: HashSet<Style>,
}

pub struct Attribute {
    pub name: Str,
    pub value: Str,
}

impl Element {
    pub fn new(tag_name: impl Into<Str>) -> Self {
        Self {
            tag_name: tag_name.into(),
            attributes: Vec::new(),
            children: Vec::new(),
            text_content: None,
            styles: HashSet::new(),
        }
    }

    pub fn with_attribute(mut self, name: impl Into<Str>, value: impl Into<Str>) -> Self {
        self.attributes.push(Attribute {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    pub fn with_child(mut self, child: Element) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_text_content(mut self, text_content: impl Into<String>) -> Self {
        self.text_content = Some(text_content.into());
        self
    }
}

impl AttributeExtension for Element {
    fn apply_style(mut self, style: impl Into<Style>) -> Self {
        self.styles.insert(style.into());
        self
    }
}
