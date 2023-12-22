mod justify_content;
mod justify_items;
mod justify_self;

pub use justify_content::*;
pub use justify_items::*;
pub use justify_self::*;

use crate::{Style, StyleError, Styleable};

/// Represents the justify styles.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Justify {
    /// justify-content: value
    Content(JustifyContent),
    /// justify-items: value
    Items(JustifyItems),
    /// justify-self: value
    Self_(JustifySelf),
}

impl From<Justify> for Style {
    fn from(value: Justify) -> Self {
        Style::Justify(value)
    }
}

impl Justify {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Justify::Content(value) => value.write_classname(stream),
            Justify::Items(value) => value.write_classname(stream),
            Justify::Self_(value) => value.write_classname(stream),
        }
    }

    pub(crate) fn write_css_statement<T>(
        &self,
        stream: &mut String,
        options: &T,
    ) -> Result<(), StyleError>
    where
        T: crate::StyleOptions,
    {
        match self {
            Justify::Content(value) => value.write_css_statement(stream, options),
            Justify::Items(value) => value.write_css_statement(stream, options),
            Justify::Self_(value) => value.write_css_statement(stream, options),
        }
    }
}

impl<T> JustifyTrait for T where T: Styleable {}

/// Justify style attributes.
pub trait JustifyTrait: Styleable {
    #[inline]
    fn justify_content(self, value: impl Into<JustifyContent>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn justify_normal(self) -> Self::Output {
        self.style(JustifyContent::Normal)
    }

    #[inline]
    fn justify_start(self) -> Self::Output {
        self.style(JustifyContent::FlexStart)
    }

    #[inline]
    fn justify_end(self) -> Self::Output {
        self.style(JustifyContent::FlexEnd)
    }

    #[inline]
    fn justify_center(self) -> Self::Output {
        self.style(JustifyContent::Center)
    }

    #[inline]
    fn justify_between(self) -> Self::Output {
        self.style(JustifyContent::SpaceBetween)
    }

    #[inline]
    fn justify_around(self) -> Self::Output {
        self.style(JustifyContent::SpaceAround)
    }

    #[inline]
    fn justify_evenly(self) -> Self::Output {
        self.style(JustifyContent::SpaceEvenly)
    }

    #[inline]
    fn justify_stretch(self) -> Self::Output {
        self.style(JustifyContent::Stretch)
    }

    #[inline]
    fn justify_items(self, value: impl Into<JustifyItems>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn justify_items_start(self) -> Self::Output {
        self.style(JustifyItems::Start)
    }

    #[inline]
    fn justify_items_end(self) -> Self::Output {
        self.style(JustifyItems::End)
    }

    #[inline]
    fn justify_items_center(self) -> Self::Output {
        self.style(JustifyItems::Center)
    }

    #[inline]
    fn justify_items_stretch(self) -> Self::Output {
        self.style(JustifyItems::Stretch)
    }

    #[inline]
    fn justify_self(self, value: impl Into<JustifySelf>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn justify_self_auto(self) -> Self::Output {
        self.style(JustifySelf::Auto)
    }

    #[inline]
    fn justify_self_start(self) -> Self::Output {
        self.style(JustifySelf::Start)
    }

    #[inline]
    fn justify_self_end(self) -> Self::Output {
        self.style(JustifySelf::End)
    }

    #[inline]
    fn justify_self_center(self) -> Self::Output {
        self.style(JustifySelf::Center)
    }

    #[inline]
    fn justify_self_stretch(self) -> Self::Output {
        self.style(JustifySelf::Stretch)
    }
}
