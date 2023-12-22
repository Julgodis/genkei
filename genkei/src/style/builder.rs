use crate::{Style, Styleable};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Default)]
pub struct StyleBuilder {
    styles: BTreeSet<Style>,
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> BTreeSet<Style> {
        self.styles
    }
}

impl Styleable for StyleBuilder {
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
