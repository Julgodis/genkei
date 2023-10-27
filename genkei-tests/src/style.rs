use genkei::{
    DefaultStyleOptions, MarginTrait, PaddingTrait, Style, StyleBuilder, StyleRenderer, Styleable, HtmlAttribute, Renderer, ColorTrait, Color,
};

#[test]
fn test_style() {
    let p = genkei::div().p(1).px(5).py(5).pt(10).pr(10).pb(10).pl(10);
    let m = genkei::div().m(1).mx(5).my(5).mt(10).mr(10).mb(10).ml(10);

    assert_eq!(p.styles.len(), 7);
    assert_eq!(m.styles.len(), 7);
}

#[test]
fn test_style_builder() {
    let styles = StyleBuilder::new().p(1).build();

    assert_eq!(styles.len(), 1);
}

#[test]
fn test_style_to_css() {
    let style = Style::Padding(genkei::Padding::All(1));
    assert_eq!(style.to_css(), Ok(".p-1{padding:0.25rem}".into()));
}

#[test]
fn test_style_render_basic() {
    let mut renderer = StyleRenderer::<DefaultStyleOptions>::new();
    renderer.include_style(genkei::Padding::All(1));
    let css = renderer.render().expect("Failed to render style");
    assert_eq!(css, ".p-1{padding:0.25rem}");
}

#[test]
fn test_style_render_state() {
    let mut renderer = StyleRenderer::<DefaultStyleOptions>::new();
    renderer.include_styles(
        StyleBuilder::new()
            .p(1)
            .hover(|style| {
                style
                    .p(2)
                    .focus(|style| style.p(3).focus_visible(|style| style.p(4)))
            })
            .build(),
    );
    let css = renderer.render().expect("Failed to render style");
    assert_eq!(
        css,
        ".p-1{padding:0.25rem};.hover\\:p-2:hover{padding:0.5rem};.focus\\:hover\\:p-3:focus:hover{padding:0.75rem};.focus\\:focus-visible\\:hover\\:p-4:focus:focus-visible:hover{padding:1rem}"
    );
}

#[test]
fn test_style_media_query() {
    let mut renderer = StyleRenderer::<DefaultStyleOptions>::new();
    renderer.include_styles(
        StyleBuilder::new()
            .p(1)
            .mq(genkei::MediaQuery::Lg, |style| style.p(2))
            .build(),
    );
    let css = renderer.render().expect("Failed to render style");
    assert_eq!(
        css,
        ".p-1{padding:0.25rem};@media(min-width:1024px){.lg\\:p-2{padding:0.5rem}}"
    );
}

#[test]
fn test_div_with_style() {
    let div = genkei::div().id("id").p(1);
    assert_eq!(Renderer::render_tag(div), "<div class=p-1 id=id></div>");
    let div1 = genkei::div().id("id").p(1).bg_color(Color::Slate050);
    let div2 = genkei::div().id("id").bg_color(Color::Slate050).p(1);
    assert_eq!(Renderer::render_tag(div1), "<div class=\"p-1 bg-slate-50\" id=id></div>");
    assert_eq!(Renderer::render_tag(div2), "<div class=\"p-1 bg-slate-50\" id=id></div>");
}

#[test]
fn test_renderer_style_options() {
    let div = genkei::div().id("id").bg_color(Color::Slate050).p(1);
    
    let mut renderer = Renderer::new()
        .styling_as_classname();
    renderer.push_tag(div.clone());
    assert_eq!(renderer.render(), "<div class=\"p-1 bg-slate-50\" id=id></div>");

    let mut renderer = Renderer::new()
        .styling_as_style();
    renderer.push_tag(div.clone());
    assert_eq!(renderer.render(), "<div id=id style=padding:0.25rem;background-color:rgb(248,250,252)></div>");
}

