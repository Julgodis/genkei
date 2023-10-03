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
macro_rules! div_group {
    () => {
        $crate::html::div()
    };
    ($($content:tt)*) => {
        element_group!($crate::html::div(), $($content)*)
    };
}

