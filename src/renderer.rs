use crate::Element;

pub trait Renderer {
    type Output;

    fn render(&self, element: &Element) -> Self::Output;
}
