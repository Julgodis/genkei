use crate::Element;
pub mod macros;
mod renderer;
pub use renderer::*;

pub fn div() -> Element {
    Element::new("div")
}
