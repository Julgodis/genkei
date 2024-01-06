use crate::StyleError;

mod complex;
mod fq;
mod simple;

pub use complex::*;
pub use fq::*;
pub use simple::*;

pub trait ColorValue {
    fn write_color_name(&self, stream: &mut String) -> Result<(), StyleError>;

    fn write_color_value<T>(&self, stream: &mut String, options: &T) -> Result<(), StyleError>
    where
        T: crate::StyleOptions;
}
