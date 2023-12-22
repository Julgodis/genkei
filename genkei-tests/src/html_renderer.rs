use genkei::{HtmlAttribute, Renderer};

#[test]
fn test_div() {
    let div = genkei::div();
    assert_eq!(
        Renderer::render_tag(div)
            .expect("Failed to render tag")
            .html(),
        "<div></div>"
    );
}

#[test]
fn test_div_with_id() {
    let div = genkei::div().id("id");
    assert_eq!(
        Renderer::render_tag(div.clone())
            .expect("Failed to render tag")
            .html(),
        "<div id=id></div>"
    );
    assert_eq!(
        div.to_html().expect("Failed to render tag"),
        "<div id=id></div>"
    );
}
