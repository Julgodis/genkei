mod attribute;
mod children;
mod color;
mod context;
mod text_content;
#[cfg(feature = "html")]
mod html;
#[cfg(feature = "style")]
mod style;

pub use attribute::*;
pub use children::*;
pub use color::*;
pub use context::*;
pub use text_content::*;
#[cfg(feature = "html")]
pub use html::*;
#[cfg(feature = "style")]
pub use style::*;

use std::borrow::Cow;
pub type Str = Cow<'static, str>;
