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
    /// display: inline-flex
    InlineFlex,
}

impl From<Display> for Style {
    fn from(value: Display) -> Self {
        Style::Display(value)
    }
}

impl Display {
    pub(crate) fn write_classname(&self, stream: &mut String) -> Result<(), StyleError> {
        match self {
            Display::None => write!(stream, "none")?,
            Display::Block => write!(stream, "block")?,
            Display::Flex => write!(stream, "flex")?,
            Display::Inline => write!(stream, "inline")?,
            Display::InlineBlock => write!(stream, "inline-block")?,
            Display::Grid => write!(stream, "grid")?,
            Display::InlineFlex => write!(stream, "inline-flex")?,
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
            Display::InlineFlex => write!(stream, "display:inline-flex")?,
        };

        Ok(())
    }
}

impl<T> DisplayTrait for T where T: Styleable {}

/// A trait for the display style attributes.
pub trait DisplayTrait: Styleable {
    #[inline]
    fn display(self, value: impl Into<Display>) -> Self::Output {
        self.style(Style::Display(value.into()))
    }

    #[inline]
    fn none(self) -> Self::Output {
        self.style(Display::None)
    }

    #[inline]
    fn block(self) -> Self::Output {
        self.style(Display::Block)
    }

    #[inline]
    fn flex(self) -> Self::Output {
        self.style(Display::Flex)
    }

    #[inline]
    fn inline(self) -> Self::Output {
        self.style(Display::Inline)
    }

    #[inline]
    fn inline_block(self) -> Self::Output {
        self.style(Display::InlineBlock)
    }

    #[inline]
    fn grid(self) -> Self::Output {
        self.style(Display::Grid)
    }
}
