#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use gatsby_transformer_markdown_wasm::{render, render_markdown};

#[wasm_bindgen_test]
pub fn test_render_markdown() {
    let markdown = "__Strong__";
    assert_eq!(render_markdown(markdown), "<p><strong>Strong</strong></p>\n");
}

#[wasm_bindgen_test]
pub fn test_render() {
    let markdown = "---\ntitle: Valid Yaml Test\n---\nsomething that's not yaml";
    let result = render(markdown);
    assert!(result.is_object());
}

#[wasm_bindgen_test]
pub fn test_render_without_frontmatter() {
    let markdown = "something that's not yaml";
    let result = render(markdown);
    assert!(result.is_object());
}
