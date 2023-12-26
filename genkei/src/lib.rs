mod attribute;
mod children;
mod color;
#[cfg(feature = "html")]
mod html;
#[cfg(feature = "style")]
mod style;
mod text_content;

pub use attribute::*;
pub use children::*;
pub use color::*;
#[cfg(feature = "html")]
pub use html::*;
#[cfg(feature = "style")]
pub use style::*;
pub use text_content::*;

use std::borrow::Cow;
pub type Str = Cow<'static, str>;
