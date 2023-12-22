use crate::{MediaQuery, State, Str, Style, StyleBuilder};

/// A trait for adding styles.
pub trait Styleable: Sized {
    type Output: Styleable;

    /// Add a style without preprocessing.
    fn style_raw(self, style: Style) -> Self::Output;

    /// Add multiple styles without preprocessing.
    fn styles_raw(self, styles: impl IntoIterator<Item = Style>) -> Self::Output;

    /// Add a style.
    fn style(self, style: impl Into<Style>) -> Self::Output {
        self.style_raw(style.into().simplify())
    }

    /// Add multiple styles.
    fn styles(self, styles: impl IntoIterator<Item = impl Into<Style>>) -> Self::Output {
        self.styles_raw(styles.into_iter().map(Into::into).map(Style::simplify))
    }

    /// Add a style for when the element is hovered.
    #[inline]
    fn hover_style(self, style: impl Into<Style>) -> Self::Output {
        self.style(State::hover(style))
    }

    /// Add styles for when the element is hovered.
    fn hover(self, style: impl FnOnce(StyleBuilder) -> StyleBuilder) -> Self::Output {
        self.styles(
            style(StyleBuilder::new())
                .build()
                .into_iter()
                .map(State::hover),
        )
    }

    /// Add a style for when the element is focused.
    #[inline]
    fn focus_style(self, style: impl Into<Style>) -> Self::Output {
        self.style(State::focus(style))
    }

    /// Add styles for when the element is focused.
    fn focus(self, style: impl FnOnce(StyleBuilder) -> StyleBuilder) -> Self::Output {
        self.styles(
            style(StyleBuilder::new())
                .build()
                .into_iter()
                .map(State::focus),
        )
    }

    /// Add a style for when the element is focus-visible.
    #[inline]
    fn focus_visible_style(self, style: impl Into<Style>) -> Self::Output {
        self.style(State::focus_visible(style))
    }

    /// Add styles for when the element is focus-visible.
    fn focus_visible(self, style: impl FnOnce(StyleBuilder) -> StyleBuilder) -> Self::Output {
        self.styles(
            style(StyleBuilder::new())
                .build()
                .into_iter()
                .map(State::focus_visible),
        )
    }

    /// Add a style for when the element is active.
    #[inline]
    fn active_style(self, style: impl Into<Style>) -> Self::Output {
        self.style(State::active(style))
    }

    /// Add styles for when the element is active.
    fn active(self, style: impl FnOnce(StyleBuilder) -> StyleBuilder) -> Self::Output {
        self.styles(
            style(StyleBuilder::new())
                .build()
                .into_iter()
                .map(State::active),
        )
    }

    #[inline]
    fn backdrop(self, style: impl FnOnce(StyleBuilder) -> StyleBuilder) -> Self::Output {
        self.styles(
            style(StyleBuilder::new())
                .build()
                .into_iter()
                .map(State::backdrop),
        )
    }

    /// Add styles for a specific media query.
    fn mq(
        self,
        media_query: impl Into<MediaQuery>,
        style: impl FnOnce(StyleBuilder) -> StyleBuilder,
    ) -> Self::Output {
        let mq = media_query.into();
        self.styles(
            style(StyleBuilder::new())
                .build()
                .into_iter()
                .map(Box::new)
                .map(|style| Style::MediaQuery(mq.clone(), style)),
        )
    }

    /// Add styles for a specific data query.
    fn dq(
        self,
        condition: impl Into<Str>,
        style: impl FnOnce(StyleBuilder) -> StyleBuilder,
    ) -> Self::Output {
        let dq = condition.into();
        self.styles(
            style(StyleBuilder::new())
                .build()
                .into_iter()
                .map(Box::new)
                .map(|style| Style::DataQuery(dq.clone(), style)),
        )
    }
}
