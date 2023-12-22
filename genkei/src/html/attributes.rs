use crate::{Attributes, IntoAttributeValue};

pub trait HtmlAttribute: Attributes {
    /// Set the `id` attribute.
    #[inline]
    fn id(self, id: impl IntoAttributeValue) -> Self::Output {
        self.attr_kv("id", id)
    }

    /// Set the `class` attribute.
    #[inline]
    fn class(self, class: impl IntoAttributeValue) -> Self::Output {
        self.attr_kv("class", class)
    }

    /// Set the `name` attribute.
    #[inline]
    fn name(self, name: impl IntoAttributeValue) -> Self::Output {
        self.attr_kv("name", name)
    }

    /// Set the `type` attribute.
    #[inline]
    fn type_(self, type_: impl IntoAttributeValue) -> Self::Output {
        self.attr_kv("type", type_)
    }

    /// Set the `value` attribute.
    #[inline]
    fn value(self, value: impl IntoAttributeValue) -> Self::Output {
        self.attr_kv("value", value)
    }

    /// Set the `placeholder` attribute.
    #[inline]
    fn placeholder(self, placeholder: impl IntoAttributeValue) -> Self::Output {
        self.attr_kv("placeholder", placeholder)
    }

    /// Set the `href` attribute.
    #[inline]
    fn href(self, href: impl IntoAttributeValue) -> Self::Output {
        self.attr_kv("href", href)
    }

    /// Set the `src` attribute.
    #[inline]
    fn src(self, src: impl IntoAttributeValue) -> Self::Output {
        self.attr_kv("src", src)
    }

    /// Set the `alt` attribute.
    #[inline]
    fn alt(self, alt: impl IntoAttributeValue) -> Self::Output {
        self.attr_kv("alt", alt)
    }

    /// Set the `disabled` attribute.
    #[inline]
    fn disabled(self) -> Self::Output {
        self.attr("disabled")
    }

    /// Set the `checked` attribute.
    #[inline]
    fn checked(self) -> Self::Output {
        self.attr("checked")
    }

    /// Set the `selected` attribute.
    #[inline]
    fn selected(self) -> Self::Output {
        self.attr("selected")
    }
}
