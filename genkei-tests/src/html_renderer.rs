use genkei::{Renderer, HtmlAttribute};

#[test]
fn test_div() {
    let div = genkei::div();
    assert_eq!(Renderer::render_tag(div), "<div></div>");
}

#[test]
fn test_div_with_id() {
    let div = genkei::div().id("id");
    assert_eq!(Renderer::render_tag(div.clone()), "<div id=id></div>");
    assert_eq!(div.to_html(), "<div id=id></div>");
}
