use crate::{Element, html::{self, macros}, div_group};

#[test]
fn test_new_element() {
    let element = Element::new("div");
    assert_eq!(element.tag_name, "div");
    assert_eq!(element.attributes.len(), 0);
    assert_eq!(element.children.len(), 0);
    assert_eq!(element.text_content, None);
    assert_eq!(element.styles.len(), 0);
}

#[test]
fn test_html_div() {
    let element = html::div();
    assert_eq!(element.tag_name, "div");
    assert_eq!(element.attributes.len(), 0);
    assert_eq!(element.children.len(), 0);
    assert_eq!(element.text_content, None);
    assert_eq!(element.styles.len(), 0);

    let element = div_group!();
    assert_eq!(element.tag_name, "div");
    assert_eq!(element.attributes.len(), 0);
    assert_eq!(element.children.len(), 0);
    assert_eq!(element.text_content, None);
    assert_eq!(element.styles.len(), 0);
}