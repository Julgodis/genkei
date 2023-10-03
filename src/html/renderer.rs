use crate::{Element, renderer::Renderer, Attribute};


pub struct HtmlRenderer {
    indent: usize,
}

impl HtmlRenderer {
    pub fn new() -> Self {
        Self { indent: 0 }
    }
}

impl Renderer for HtmlRenderer {
    type Output = String;

    fn render(&self, element: &Element) -> Self::Output {
        let mut output = String::new();

        output.push_str(&format!(
            "{}<{}",
            " ".repeat(self.indent),
            element.tag_name
        ));

        for Attribute { name, value } in &element.attributes {
            output.push_str(&format!(" {}=\"{}\"", name, value));
        }

        if element.children.is_empty() {
            output.push_str(" />");
        } else {
            output.push_str(">\n");

            for child in &element.children {
                output.push_str(&self.render(child));
                output.push('\n');
            }

            output.push_str(&format!("{}</{}>", " ".repeat(self.indent), element.tag_name));
        }

        output
    }
}