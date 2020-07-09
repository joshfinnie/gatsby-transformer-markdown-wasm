use yaml_rust::YamlEmitter;
use serde::Serialize;
use frontmatter::parse_and_find_content;
use pulldown_cmark::{html, Options, Parser};

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render_markdown(markdown_input: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    return html_output;
}

#[derive(Serialize)]
pub struct Data {
    data: serde_json::Value,
    content: String,
}

#[wasm_bindgen]
pub fn render(markdown_input: &str) -> JsValue {
    console_error_panic_hook::set_once(); // Helps return errors for debugging

    let parsed = parse_and_find_content(markdown_input);
    let (frontmatter, content) = parsed.unwrap();

    if frontmatter.is_some() {
        let mut value = String::new();
        let mut emitter = YamlEmitter::new(&mut value);
        emitter.dump(&frontmatter.unwrap()).unwrap();
        let yaml_contents: serde_json::Value = serde_yaml::from_str(&value).unwrap();
        let data = Data{data: yaml_contents, content: render_markdown(content)};
        return JsValue::from_serde(&data).unwrap();
    } else {
        let yaml_contents: serde_json::Value = serde_json::Value::Null;
        let data = Data{data: yaml_contents, content: render_markdown(content)};
        return JsValue::from_serde(&data).unwrap();
    }
}
