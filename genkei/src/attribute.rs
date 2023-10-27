use crate::Str;

/// An attribute.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Attribute {
    /// A key attribute, without a value.
    Key(Str),
    /// A key-value attribute.
    KeyValue(Str, Str),
}

impl Attribute {
    pub fn key(name: impl Into<Str>) -> Self {
        Self::Key(name.into())
    }

    pub fn key_value(name: impl Into<Str>, value: impl IntoAttributeValue) -> Self {
        Self::KeyValue(name.into(), value.into_attribute_value())
    }

    pub fn get_key(&self) -> &Str {
        match self {
            Self::Key(name) => name,
            Self::KeyValue(name, _) => name,
        }
    }

    pub fn get_value(&self) -> Option<&Str> {
        match self {
            Self::Key(_) => None,
            Self::KeyValue(_, value) => Some(value),
        }
    }

    pub fn is_style(&self) -> bool {
        self.get_key() == "style"
    }

    pub fn is_class(&self) -> bool {
        self.get_key() == "class"
    }
}

/// A trait for converting a value into an attribute.
pub trait IntoAttribute {
    fn into_attribute(self) -> Attribute;
}

impl<K, V> IntoAttribute for (K, V)
where
    K: Into<Str>,
    V: IntoAttributeValue,
{
    fn into_attribute(self) -> Attribute {
        Attribute::key_value(self.0, self.1)
    }
}

impl<K> IntoAttribute for (K,)
where
    K: Into<Str>,
{
    fn into_attribute(self) -> Attribute {
        Attribute::key(self.0)
    }
}

impl IntoAttribute for Str {
    fn into_attribute(self) -> Attribute {
        Attribute::key(self)
    }
}

impl IntoAttribute for &'static str {
    fn into_attribute(self) -> Attribute {
        Attribute::key(self)
    }
}

impl IntoAttribute for String {
    fn into_attribute(self) -> Attribute {
        Attribute::key(self)
    }
}

impl IntoAttribute for Attribute {
    fn into_attribute(self) -> Attribute {
        self
    }
}

/// A trait for converting a value into an attribute value.
pub trait IntoAttributeValue {
    fn into_attribute_value(self) -> Str;
}

impl IntoAttributeValue for Str {
    fn into_attribute_value(self) -> Str {
        self
    }
}

impl IntoAttributeValue for &'static str {
    fn into_attribute_value(self) -> Str {
        self.into()
    }
}

impl IntoAttributeValue for String {
    fn into_attribute_value(self) -> Str {
        self.into()
    }
}

impl IntoAttributeValue for bool {
    fn into_attribute_value(self) -> Str {
        if self {
            "true".into()
        } else {
            "false".into()
        }
    }
}

/// A trait for adding attributes.
pub trait Attributes
where
    Self: Sized,
{
    /// Add an attribute.
    fn attr(self, attribute: impl IntoAttribute) -> Self;

    /// Add key-value attribute.
    fn attr_kv(self, name: impl Into<Str>, value: impl IntoAttributeValue) -> Self {
        self.attr(Attribute::key_value(name, value))
    }
}
