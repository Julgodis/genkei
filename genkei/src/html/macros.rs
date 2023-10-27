#[macro_export]
#[warn(unused_macros)]
macro_rules! element_group {
    ($element:tt, $($content:tt)*) => {
        {
            $(
                $element = $element.with_child($content);
            )*
            element
        }
    };
}

#[macro_export]
#[warn(unused_macros)]
macro_rules! div_group {
    () => {
        $crate::html::div()
    };
    ($($content:tt)*) => {
        element_group!($crate::html::div(), $($content)*)
    };
}

