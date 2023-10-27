use crate::Str;

/// A trait for adding text content.
pub trait TextContent {
    /// Add text content.
    fn text_content(self, content: impl Into<Str>) -> Self;
}
