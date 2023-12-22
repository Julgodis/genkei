mod align_content;
mod align_items;
mod align_self;

pub use align_content::*;
pub use align_items::*;
pub use align_self::*;

use crate::{Style, StyleError, Styleable};

/// Represents the align styles.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Align {
    /// align-content: value
    Content(AlignContent),
    /// align-items: value
    Items(AlignItems),
    /// align-self: value
    Self_(AlignSelf),
}

impl From<Align> for Style {
    fn from(value: Align) -> Self {
        Style::Align(value)
    }
}

impl Align {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Align::Content(value) => value.write_classname(stream),
            Align::Items(value) => value.write_classname(stream),
            Align::Self_(value) => value.write_classname(stream),
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
            Align::Content(value) => value.write_css_statement(stream, options),
            Align::Items(value) => value.write_css_statement(stream, options),
            Align::Self_(value) => value.write_css_statement(stream, options),
        }
    }
}

impl<T> AlignTrait for T where T: Styleable {}

/// Align style attributes.
pub trait AlignTrait: Styleable {
    #[inline]
    fn align_content(self, value: impl Into<AlignContent>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn content_normal(self) -> Self::Output {
        self.style(AlignContent::Normal)
    }

    #[inline]
    fn content_center(self) -> Self::Output {
        self.style(AlignContent::Center)
    }

    #[inline]
    fn content_start(self) -> Self::Output {
        self.style(AlignContent::FlexStart)
    }

    #[inline]
    fn content_end(self) -> Self::Output {
        self.style(AlignContent::FlexEnd)
    }

    #[inline]
    fn content_between(self) -> Self::Output {
        self.style(AlignContent::SpaceBetween)
    }

    #[inline]
    fn content_around(self) -> Self::Output {
        self.style(AlignContent::SpaceAround)
    }

    #[inline]
    fn content_evenly(self) -> Self::Output {
        self.style(AlignContent::SpaceEvenly)
    }

    #[inline]
    fn content_baseline(self) -> Self::Output {
        self.style(AlignContent::Baseline)
    }

    #[inline]
    fn content_stretch(self) -> Self::Output {
        self.style(AlignContent::Stretch)
    }

    #[inline]
    fn align_items(self, value: impl Into<AlignItems>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn items_start(self) -> Self::Output {
        self.style(AlignItems::FlexStart)
    }

    #[inline]
    fn items_end(self) -> Self::Output {
        self.style(AlignItems::FlexEnd)
    }

    #[inline]
    fn items_center(self) -> Self::Output {
        self.style(AlignItems::Center)
    }

    #[inline]
    fn items_baseline(self) -> Self::Output {
        self.style(AlignItems::Baseline)
    }

    #[inline]
    fn items_stretch(self) -> Self::Output {
        self.style(AlignItems::Stretch)
    }

    #[inline]
    fn align_self(self, value: impl Into<AlignSelf>) -> Self::Output {
        self.style(value.into())
    }

    #[inline]
    fn self_auto(self) -> Self::Output {
        self.style(AlignSelf::Auto)
    }

    #[inline]
    fn self_start(self) -> Self::Output {
        self.style(AlignSelf::FlexStart)
    }

    #[inline]
    fn self_end(self) -> Self::Output {
        self.style(AlignSelf::FlexEnd)
    }

    #[inline]
    fn self_center(self) -> Self::Output {
        self.style(AlignSelf::Center)
    }

    #[inline]
    fn self_baseline(self) -> Self::Output {
        self.style(AlignSelf::Baseline)
    }

    #[inline]
    fn self_stretch(self) -> Self::Output {
        self.style(AlignSelf::Stretch)
    }
}
