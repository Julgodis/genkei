use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{ColorValue, Str, StyleError, StyleOptions};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComplexColor {
    Custom(String),
}

impl ComplexColor {
    pub fn to_classname(&self) -> Str {
        match self {
            Self::Custom(s) => {
                let mut hasher = DefaultHasher::new();
                s.hash(&mut hasher);
                let hash = hasher.finish();
                Str::from(format!("custom-{}", hash))
            }
        }
    }

    pub(crate) fn write_color_name(&self, stream: &mut String) -> Result<(), StyleError> {
        stream.push_str(&self.to_classname());
        Ok(())
    }

    pub(crate) fn write_css_value<T: StyleOptions>(
        &self,
        stream: &mut String,
        _options: &T,
    ) -> Result<(), StyleError> {
        match self {
            Self::Custom(s) => stream.push_str(s),
        };
        Ok(())
    }
}

impl ColorValue for ComplexColor {
    fn write_color_name(&self, stream: &mut String) -> Result<(), StyleError> {
        self.write_color_name(stream)
    }

    fn write_color_value<T>(&self, stream: &mut String, options: &T) -> Result<(), StyleError>
    where
        T: crate::StyleOptions,
    {
        self.write_css_value(stream, options)
    }
}
