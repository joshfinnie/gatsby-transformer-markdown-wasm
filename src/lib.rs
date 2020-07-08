use yaml_rust::YamlEmitter;
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

#[wasm_bindgen]
pub fn render(markdown_input: &str) -> String {
    console_error_panic_hook::set_once();
    let parsed = parse_and_find_content(markdown_input);
    let (matter, content) = parsed.unwrap();

    if matter.is_some() {
        let mut value = String::new();
        let mut emitter = YamlEmitter::new(&mut value);
        emitter.dump(&matter.unwrap()).unwrap();
        let yaml_contents: serde_json::Value = serde_yaml::from_str(&value).unwrap();

        let markdown_content: String = render_markdown(content).to_string();

        return format!("{{\"data\": {}, \"content\": \"{}\"}}", yaml_contents.to_string(), &markdown_content[..markdown_content.len()-1]);
    } else {
        let markdown_content: String = render_markdown(content).to_string();
        return format!("{{\"data\": {}, \"content\": \"{}\"}}", "null", &markdown_content[..markdown_content.len()-1]);
    }
}
