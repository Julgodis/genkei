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
    /// Create a key attribute. This attribute does not have a value.
    pub fn key(name: impl Into<Str>) -> Self {
        Self::Key(name.into())
    }

    /// Create a key-value attribute.
    pub fn key_value(name: impl Into<Str>, value: impl IntoAttributeValue) -> Self {
        Self::KeyValue(name.into(), value.into_attribute_value())
    }

    /// Get the key of the attribute.
    pub fn get_key(&self) -> &Str {
        match self {
            Self::Key(name) => name,
            Self::KeyValue(name, _) => name,
        }
    }

    /// Get the value of the attribute if it exists.
    pub fn get_value(&self) -> Option<&Str> {
        match self {
            Self::Key(_) => None,
            Self::KeyValue(_, value) => Some(value),
        }
    }
}

/// A trait for converting a value into an attribute.
pub trait IntoAttribute {
    /// Convert a value into an attribute.
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
    /// Convert a value into an attribute value.
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

impl IntoAttributeValue for usize {
    fn into_attribute_value(self) -> Str {
        self.to_string().into()
    }
}

impl IntoAttributeValue for i64 {
    fn into_attribute_value(self) -> Str {
        self.to_string().into()
    }
}

impl IntoAttributeValue for i32 {
    fn into_attribute_value(self) -> Str {
        self.to_string().into()
    }
}

impl IntoAttributeValue for i16 {
    fn into_attribute_value(self) -> Str {
        self.to_string().into()
    }
}

impl IntoAttributeValue for i8 {
    fn into_attribute_value(self) -> Str {
        self.to_string().into()
    }
}

impl IntoAttributeValue for u64 {
    fn into_attribute_value(self) -> Str {
        self.to_string().into()
    }
}

impl IntoAttributeValue for u32 {
    fn into_attribute_value(self) -> Str {
        self.to_string().into()
    }
}

impl IntoAttributeValue for u16 {
    fn into_attribute_value(self) -> Str {
        self.to_string().into()
    }
}

impl IntoAttributeValue for u8 {
    fn into_attribute_value(self) -> Str {
        self.to_string().into()
    }
}

/// A trait for elements that can have attributes.
pub trait Attributes
where
    Self: Sized,
{
    type Output;

    /// Add an attribute.
    fn attr(self, attribute: impl IntoAttribute) -> Self::Output;

    /// Add key-value attribute.
    fn attr_kv(self, name: impl Into<Str>, value: impl IntoAttributeValue) -> Self::Output {
        self.attr(Attribute::key_value(name, value))
    }
}
