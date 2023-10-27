use crate::{Style, StyleError, Styleable};
use std::fmt::Write;

/// Represents the display property style.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Display {
    /// display: none
    None,
    /// display: block
    Block,
    /// display: flex
    Flex,
    /// display: inline
    Inline,
    /// display: inline-block
    InlineBlock,
    /// display: grid
    Grid,
}

impl From<Display> for Style {
    fn from(value: Display) -> Self {
        Style::Display(value)
    }
}

impl Display {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Display::None => write!(stream, "d-none")?,
            Display::Block => write!(stream, "d-block")?,
            Display::Flex => write!(stream, "d-flex")?,
            Display::Inline => write!(stream, "d-inline")?,
            Display::InlineBlock => write!(stream, "d-inline-block")?,
            Display::Grid => write!(stream, "d-grid")?,
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
            Display::None => write!(stream, "display:none")?,
            Display::Block => write!(stream, "display:block")?,
            Display::Flex => write!(stream, "display:flex")?,
            Display::Inline => write!(stream, "display:inline")?,
            Display::InlineBlock => write!(stream, "display:inline-block")?,
            Display::Grid => write!(stream, "display:grid")?,
        };

        Ok(())
    }
}

impl<T> DisplayTrait for T where T: Styleable {}

/// A trait for the display style attributes.
pub trait DisplayTrait: Styleable {
    /// Sets the display style attribute.
    #[inline]
    fn display(self, value: impl Into<Display>) -> Self {
        self.style(Style::Display(value.into()))
    }

    /// Sets the display style attribute to none.
    #[inline]
    fn none(self) -> Self {
        self.style(Display::None)
    }

    /// Sets the display style attribute to block.
    #[inline]
    fn block(self) -> Self {
        self.style(Display::Block)
    }

    /// Sets the display style attribute to flex.
    #[inline]
    fn flex(self) -> Self {
        self.style(Display::Flex)
    }

    /// Sets the display style attribute to inline.
    #[inline]
    fn inline(self) -> Self {
        self.style(Display::Inline)
    }

    /// Sets the display style attribute to inline-block.
    #[inline]
    fn inline_block(self) -> Self {
        self.style(Display::InlineBlock)
    }

    /// Sets the display style attribute to grid.
    #[inline]
    fn grid(self) -> Self {
        self.style(Display::Grid)
    }
}
