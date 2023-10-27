use crate::{Attributes, IntoAttributeValue, Str};

pub trait HtmxAttribute: Attributes {
    /// Set the `hx-boost` attribute.
    #[inline]
    fn hx_boost(self, value: bool) -> Self {
        self.attr_kv("hx-boost", value)
    }

    /// Set the `hx-get` attribute.
    #[inline]
    fn hx_get(self, value: impl Into<Str>) -> Self {
        self.attr_kv("hx-get", Into::<Str>::into(value))
    }

    /// Set the `hx-post` attribute.
    #[inline]
    fn hx_post(self, value: impl Into<Str>) -> Self {
        self.attr_kv("hx-post", Into::<Str>::into(value))
    }

    /// Set the `hx-put` attribute.
    #[inline]
    fn hx_put(self, value: impl Into<Str>) -> Self {
        self.attr_kv("hx-put", Into::<Str>::into(value))
    }

    /// Set the `hx-delete` attribute.
    #[inline]
    fn hx_delete(self, value: impl Into<Str>) -> Self {
        self.attr_kv("hx-delete", Into::<Str>::into(value))
    }

    /// Set the `hx-swap` attribute.
    #[inline]
    fn hx_swap(self, value: impl Into<HxSwap>) -> Self {
        self.attr_kv("hx-swap", Into::<HxSwap>::into(value))
    }
}

/// The hx-swap attribute allows you to specify how the response will be swapped in relative to the target of an AJAX request.
pub enum HxSwap {
    /// Replace the inner html of the target element.
    InnerHtml,
    /// Replace the entire target element with the response.
    OuterHtml,
    /// Insert the response before the target element.
    BeforeBegin,
    /// Insert the response before the first child of the target element.
    AfterBegin,
    /// Insert the response after the last child of the target element.
    BeforeEnd,
    /// Insert the response after the target element.
    AfterEnd,
    /// Deletes the target element regardless of the response.
    Delete,
    /// Does not append content from response (out of band items will still be processed).
    None,
}

impl IntoAttributeValue for HxSwap {
    fn into_attribute_value(self) -> Str {
        match self {
            HxSwap::InnerHtml => "innerHTML",
            HxSwap::OuterHtml => "outerHTML",
            HxSwap::BeforeBegin => "beforebegin",
            HxSwap::AfterBegin => "afterbegin",
            HxSwap::BeforeEnd => "beforeend",
            HxSwap::AfterEnd => "afterend",
            HxSwap::Delete => "delete",
            HxSwap::None => "none",
        }
        .into()
    }
}
