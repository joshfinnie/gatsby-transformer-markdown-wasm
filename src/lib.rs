use regex::Regex;
use serde::Serialize;
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

fn get_frontmatter(markdown_input: &str) -> (Option<String>, &str) {
    let re = Regex::new(r#"^(---[[:space:]]?)([\w\n:"' -/]*)?([[:space:]]?---)\n*(.*)"#).unwrap();

    let caps = re.captures(markdown_input);
    let (frontmatter, content) = match caps.is_some() {
        true => {
            let caps = caps.unwrap();
            (Some(caps.get(2).map_or("", |m| m.as_str()).to_string()), caps.get(4).map_or("", |m| m.as_str()))
        },
        false => (None, markdown_input),
    };

    (frontmatter, content)
}

#[derive(Serialize)]
pub struct Data {
    data: serde_json::Value,
    content: String,
}

#[wasm_bindgen]
pub fn render(markdown_input: &str, parse: Option<bool>) -> JsValue {
    console_error_panic_hook::set_once(); // Helps return errors for debugging

    let (frontmatter, content) = get_frontmatter(markdown_input);

    if frontmatter.is_some() {
        let yaml_contents: serde_json::Value = serde_yaml::from_str(&frontmatter.unwrap()).unwrap();
        let data = Data{data: yaml_contents, content: match parse.unwrap_or(false) {
            true => render_markdown(content),
            false => content.to_string(),
        }};
        return JsValue::from_serde(&data).unwrap();
    } else {
        let yaml_contents: serde_json::Value = serde_json::Value::Null;
        let data = Data{data: yaml_contents, content: match parse.unwrap_or(false) {
            true => render_markdown(content),
            false => content.to_string(),
        }};
        return JsValue::from_serde(&data).unwrap();
    }
}
