use crate::Str;

/// A trait for elements that can have text content.
pub trait TextContent {
    type Output;

    /// Add text content.
    fn text_content(self, content: impl Into<Str>) -> Self::Output;
}
