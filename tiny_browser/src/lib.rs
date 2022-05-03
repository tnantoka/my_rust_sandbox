use wasm_bindgen::prelude::*;

mod css;
mod dom;
mod html;
mod layout;
mod painting;
mod style;

#[wasm_bindgen]
pub fn render(input_html: &str, input_css: &str) -> JsValue {
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let root_node = html::parse(input_html.to_string());
    let stylesheet = css::parse(input_css.to_string());
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let canvas = painting::paint(&layout_root, viewport.content);

    JsValue::from_serde(&canvas).unwrap()
}
