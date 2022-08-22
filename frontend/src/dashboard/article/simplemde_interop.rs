use gloo::utils::document;
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use yew_interop::declare_resources;

declare_resources!(
    simplemde
    "/static/js/simplemde/simplemde.min.js"
    // "https://cdn.jsdelivr.net/simplemde/latest/simplemde.min.js"
    "/static/js/simplemde/simplemde.min.css"
);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = SimpleMDE)]
    pub type SimpleMDE;

    #[wasm_bindgen(constructor, js_class = "SimpleMDE")]
    pub fn new(config: &JsValue) -> SimpleMDE;
}

pub fn create_editor() {
    let config = Object::new();

    Reflect::set(
        &config,
        &"element".into(),
        &document().get_element_by_id("editor").unwrap(),
    ).ok();

    SimpleMDE::new(&config);
}