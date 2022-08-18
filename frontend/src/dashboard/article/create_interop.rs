use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use yew_interop::declare_resources;

declare_resources!(
    simplemde
    "/static/js/simplemde/simplemde.min.js"
    "/static/js/simplemde/simplemde.min.css"
);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "SimpleMDE")]
    pub type SimpleMDE;

    #[wasm_bindgen(constructor, js_class = "SimpleMDE")]
    pub fn new(config: &JsValue) -> SimpleMDE;
}
