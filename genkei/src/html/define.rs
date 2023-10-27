#[macro_export]
macro_rules! tag_def {
    ($element:ident) => {
        pub fn $element() -> $crate::html::Tag {
            $crate::html::Tag::new(stringify!($element))
        }
    };
}

#[macro_export]
macro_rules! tag_def_custom {
    ($element:ident, $self_closing:expr) => {
        pub fn $element() -> $crate::html::Tag {
            $crate::html::Tag::new_with_options(stringify!($element), $self_closing)
        }
    };
}
